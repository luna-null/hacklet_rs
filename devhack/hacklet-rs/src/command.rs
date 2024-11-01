use argh::FromArgs;
use log::{info, debug};
use crate::dongle::Dongle;

/// Hacklet CLI - Manage your smart sockets and devices.
#[derive(FromArgs)]
pub struct Hacklet {
    /// enables debug logging
    #[argh(switch, short = 'd')]
    pub debug: bool,

    #[argh(subcommand)]
    pub command: Commands,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Commands {
    On(OnCommand),
    Off(OffCommand),
    Read(ReadCommand),
    Commission(CommissionCommand),
}

/// Turn on the specified socket.
#[derive(FromArgs)]
#[argh(subcommand, name = "on")]
pub struct OnCommand {
    /// the network id (ex. 0x1234)
    #[argh(option, short = 'n')]
    pub network: String,

    /// the socket id (ex. 0)
    #[argh(option, short = 's')]
    pub socket: String,
}

/// Turn off the specified socket.
#[derive(FromArgs)]
#[argh(subcommand, name = "off")]
pub struct OffCommand {
    /// the network id (ex. 0x1234)
    #[argh(option, short = 'n')]
    pub network: String,

    /// the socket id (ex. 0)
    #[argh(option, short = 's')]
    pub socket: String,
}

/// Read all available samples from the specified socket.
#[derive(FromArgs)]
#[argh(subcommand, name = "read")]
pub struct ReadCommand {
    /// the network id (ex. 0x1234)
    #[argh(option, short = 'n')]
    pub network: String,

    /// the socket id (ex. 0)
    #[argh(option, short = 's')]
    pub socket: String,
}

/// Add a new device to the network.
#[derive(FromArgs)]
#[argh(subcommand, name = "commission")]
pub struct CommissionCommand {}

pub fn command() {
    let args: Hacklet = argh::from_env();
    
    // Initialize the dongle
    Dongle::open(|dongle| {
        // Enable debug logging if specified
        if args.debug {
            debug!("Debug logging enabled");
        }

        // Match subcommands
        match args.command {
            Commands::On(cmd) => {
                let network_id = u16::from_str_radix(&cmd.network[2..], 16).unwrap();
                let socket_id = cmd.socket.parse::<u16>().unwrap();
                
                dongle.lock_network();
                dongle.select_network(network_id);
                dongle.switch(network_id, socket_id, true);
                info!("Turned on network 0x{:x}, socket {}", network_id, socket_id);
            }
            Commands::Off(cmd) => {
                let network_id = u16::from_str_radix(&cmd.network[2..], 16).unwrap();
                let socket_id = cmd.socket.parse::<u16>().unwrap();
                
                dongle.lock_network();
                dongle.select_network(network_id);
                dongle.switch(network_id, socket_id, false);
                info!("Turned off network 0x{:x}, socket {}", network_id, socket_id);
            }
            Commands::Read(cmd) => {
                let network_id = u16::from_str_radix(&cmd.network[2..], 16).unwrap();
                let socket_id = cmd.socket.parse::<u16>().unwrap();
                
                dongle.lock_network();
                dongle.select_network(network_id);
                let _samples = dongle.request_samples(network_id, socket_id);
                info!("Read samples from network 0x{:x}, socket {}", network_id, socket_id);
            }
            Commands::Commission(_) => {
                dongle.commission();
                info!("Commissioning new devices...");
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::command::*;
    use mockall::predicate::*;
    use mockall::mock;

    // Mock the Dongle struct
    mock! {
        pub Dongle {
            fn lock_network(&self);
            fn select_network(&self, network_id: u16);
            fn switch(&self, network_id: u16, socket_id: u16, state: bool);
            fn request_samples(&self, network_id: u16, socket_id: u16);
            fn commission(&self);
        }
    }

    #[test]
    fn test_turn_on_socket() {
        let mut dongle = MockDongle::new();

        dongle.expect_lock_network().times(1);
        dongle.expect_select_network().with(eq(0x0010)).times(1);
        dongle.expect_switch().with(eq(0x0010), eq(1), eq(true)).times(1);

        // Create a dummy command object to run the command method
        let args = Hacklet {
            debug: false,
            command: Commands::On(OnCommand {
                network: "0x0010".to_string(),
                socket: "1".to_string(),
            }),
        };

        // Call the command function
        command();
    }

    #[test]
    fn test_turn_off_socket() {
        let mut dongle = MockDongle::new();

        dongle.expect_lock_network().times(1);
        dongle.expect_select_network().with(eq(0x0010)).times(1);
        dongle.expect_switch().with(eq(0x0010), eq(0), eq(false)).times(1);

        let args = Hacklet {
            debug: false,
            command: Commands::Off(OffCommand {
                network: "0x0010".to_string(),
                socket: "0".to_string(),
            }),
        };

        command();
    }

    #[test]
    fn test_read_socket() {
        let mut dongle = MockDongle::new();

        dongle.expect_lock_network().times(1);
        dongle.expect_select_network().with(eq(0x0010)).times(1);
        dongle.expect_request_samples().with(eq(0x0010), eq(1)).times(1);

        let args = Hacklet {
            debug: false,
            command: Commands::Read(ReadCommand {
                network: "0x0010".to_string(),
                socket: "1".to_string(),
            }),
        };

        command();
    }

    #[test]
    fn test_commission_device() {
        let mut dongle = MockDongle::new();

        dongle.expect_commission().times(1);

        let args = Hacklet {
            debug: false,
            command: Commands::Commission(CommissionCommand {}),
        };

        command();
    }
}
