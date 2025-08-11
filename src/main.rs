mod cli;

use cli::{Cli, Mode};
use linker::Link;
use linker::Unlink;

fn main() {
    let cli = Cli::parsed();

    match cli.mode {
        Mode::Link => {
            let link = Link::new(cli.dir(), cli.target(), cli.simulate);

            match cli.package {
                Some(package) => link.handle_package(&package),
                None => link.handle_packages(),
            }
        }

        Mode::Unlink => {
            let unlink = Unlink::new(cli.dir(), cli.target(), cli.simulate);

            match cli.package {
                Some(package) => unlink.handle_package(&package),
                None => unlink.handle_packages(),
            }
        }

        Mode::Relink => {
            let link = Link::new(cli.dir(), cli.target(), cli.simulate);
            let unlink = Unlink::new(cli.dir(), cli.target(), cli.simulate);

            match cli.package {
                Some(package) => {
                    unlink.handle_package(&package);
                    link.handle_package(&package);
                }
                None => {
                    unlink.handle_packages();
                    link.handle_packages();
                }
            }
        }
    }
}
