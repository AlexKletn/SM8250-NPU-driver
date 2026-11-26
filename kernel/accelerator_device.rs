// SPDX-License-Identifier: GPL-2.0-only

use kernel::{
    device::{Bound, Core, Device},
    error::{
        Error, Result,
        code::{EINVAL, EIO, ENODEV, ETIMEDOUT},
    },
    io::{Io, IoBase, mem::IoMem},
    of, platform,
    prelude::*,
    str::CString,
    time::{Delta, Instant, Monotonic, delay::fsleep},
};

use crate::{core_mmio::CoreMmio, linux_boot_clocks::LinuxBootClocks, linux_cache::LinuxCache};
use crate::{firmware::Mailbox, ipc_mmio::IpcMmio, linux_pas::LinuxPas, linux_power::LinuxPower};

pub(crate) struct AcceleratorDevice<'bound> {
    core: IoMem<'bound>,
    table_address: u32,
    tcm: IoMem<'bound>,
    apss_shared: IoMem<'bound>,
}

kernel::of_device_table!(OF_TABLE, (), [(of::DeviceId::new(c"qcom,sm8250-npu"), ())]);

impl platform::Driver for AcceleratorDevice<'static> {
    type IdInfo = ();
    type Data<'bound> = AcceleratorDevice<'bound>;
    const SUPPRESS_BIND_ATTRS: bool = true;
    const OF_ID_TABLE: Option<of::IdTable<Self::IdInfo>> = Some(&OF_TABLE);

    fn probe<'bound>(
        device: &'bound platform::Device<Core<'_>>,
        _info: Option<&'bound Self::IdInfo>,
    ) -> impl PinInit<Self::Data<'bound>, Error> + 'bound {
        let accelerator = AcceleratorDevice::map(device)?;
        crate::linux_dma_buffer::LinuxDmaBuffer::initialize_device(device.as_ref())?;
        accelerator.check_firmware_boot(device.as_ref())?;
        Ok(accelerator)
    }
}

impl<'bound> AcceleratorDevice<'bound> {
    fn check_firmware_boot(&self, dev: &Device<Bound>) -> Result {
        let firmware = dev
            .fwnode()
            .ok_or(ENODEV)?
            .property_read::<CString>(c"firmware-name")
            .required_by(dev)?;
        let mut validation = crate::graph_validation::GraphValidation::prepare(dev)?;
        let power = LinuxPower::enable(dev)?;
        let clocks = LinuxBootClocks::enable(dev)?;
        let core = (&self.core).as_view();
        let cache = LinuxCache::enable(dev, core)?;
        let mut mailbox = CoreMmio(core);
        Mailbox::reset_before_boot(&mut mailbox).map_err(|_| EIO)?;
        let mut ipc = IpcMmio::new((&self.tcm).as_view(), (&self.apss_shared).as_view())?;
        ipc.initialize_table().map_err(|_| EIO)?;

        let starting = Mailbox::publish_table(&mut mailbox, self.table_address)
            .map_err(|_| EIO)
            .and_then(|_| {
                let mut pas = LinuxPas::load(dev, &firmware)?;
                pas.start()?;
                Ok(pas)
            });
        let mut pas = match starting {
            Ok(pas) => pas,
            Err(error) => {
                if Mailbox::clear_publication(&mut mailbox).is_err() {
                    dev_err!(
                        dev,
                        "Could not clear the NPU mailbox after failed startup\n"
                    );
                }
                return Err(error);
            }
        };

        let ready = Self::wait_for_firmware(&mut mailbox).and_then(|()| {
            crate::firmware_exchange::check_firmware_ipc(dev, &mut mailbox, &mut ipc)?;
            if let Some(test) = &mut validation {
                test.run(dev, &mut ipc)?;
            }
            Ok(())
        });
        if let Err(error) = pas.stop() {
            dev_err!(
                dev,
                "PAS could not stop NPU firmware: {error:?}; retaining power and clocks\n"
            );
            LinuxPas::pin_module_after_failed_stop();
            // Secure firmware can still access these resources until it is stopped.
            core::mem::forget((pas, cache, clocks, power, validation));
            return Ok(());
        }
        drop((pas, cache, clocks));
        let power_off = power.disable();
        ready.and(power_off)?;
        dev_info!(dev, "NPU firmware boot and shutdown completed\n");
        Ok(())
    }

    fn wait_for_firmware(mailbox: &mut CoreMmio<'_>) -> Result {
        let start = Instant::<Monotonic>::now();
        while start.elapsed() < Delta::from_millis(5000) {
            if Mailbox::firmware_ready(mailbox).map_err(|_| EIO)? {
                return Ok(());
            }
            fsleep(Delta::from_millis(10));
        }
        Err(ETIMEDOUT)
    }

    fn map(device: &'bound platform::Device<Bound>) -> Result<Self> {
        let tcm_resource = device.resource_by_name(c"tcm").ok_or(ENODEV)?;
        let table_address = Mailbox::table_address(tcm_resource.start()).map_err(|_| EINVAL)?;
        let core = device.io_request_by_name(c"core").ok_or(ENODEV)?.iomap()?;
        if (&core).size() < Mailbox::REGISTER_BYTES {
            return Err(EINVAL);
        }
        let tcm = device.io_request_by_name(c"tcm").ok_or(ENODEV)?.iomap()?;
        let apss_shared = device
            .io_request_by_name(c"apss_shared")
            .ok_or(ENODEV)?
            .iomap()?;
        IpcMmio::new((&tcm).as_view(), (&apss_shared).as_view())?;

        Ok(Self {
            core: core,
            table_address: table_address,
            tcm: tcm,
            apss_shared: apss_shared,
        })
    }
}
