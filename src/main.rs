extern crate config;
extern crate matrix_bot_api;

mod translate;

use matrix_bot_api::MatrixBot;
use matrix_bot_api::handlers::StatelessHandler;
use translate::translateme;

fn main() {

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("botconfig")).unwrap();

    let user = settings.get_str("user").unwrap();
    let password  = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();
    let prefix = settings.get_str("prefix").unwrap();
    let command = settings.get_str("command").unwrap();

    let mut tran = StatelessHandler::new();
    tran.set_cmd_prefix(&format!("{}", prefix));
    tran.register_handle(&format!("{}", command), translateme);

    let bot = MatrixBot::new(tran);

    bot.run(&user, &password, &homeserver_url);
}
