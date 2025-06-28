use teloxide::prelude::*;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::dptree;

mod handlers;

#[tokio::main]
async fn main()  {
    let bot = Bot::new("your token here");
    
    let schema = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<handlers::Command>()
                .endpoint(handlers::handle_commands),
        )
        .branch(
            Update::filter_message()
                .endpoint(handlers::text::handle_text),
        );

    Dispatcher::builder(bot, schema)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}