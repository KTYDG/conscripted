use {chrono::NaiveDate, conscripted_bot::*, dotenv, teloxide::prelude::*, tokio};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Could not set env variables");
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |mes: Message, bot: AutoSend<Bot>| async move {
        let m = mes.text().unwrap_or_default().split_whitespace().collect::<Vec<&str>>();
        let mut m = m.iter();
        match m.next() {
            Some(&"/start") | Some(&"/help") => {
                bot.send_message(mes.chat.id,
                    "Данный бот хранит в себе 290655 фамилий людей, призванных из-за частичной мобилизации\n/find - поиск человека по фио и дате рождения: /find ФИО д.м.г\n/fio - поиск человека по фио\n/help - повтор данного сообщения
                ").await?
            },
            Some(&"/find") => {
                if m.len() != 4 {
                    bot.send_message(mes.chat.id, "Неверный формат ввода").await?
                } else {
                    let fio = format!("{} {} {}",
                        m.next().unwrap_or(&"A"),
                        m.next().unwrap_or(&"A"),
                        m.next().unwrap_or(&"A")).to_uppercase();
                    match NaiveDate::parse_from_str(m.next().unwrap_or(&"0"), "%d.%m.%Y") {
                        Ok(bd) => {
                            let connection = &mut establish_connection();
                            println!("{} {}", fio, from_date(bd));
                            if find_man(connection, fio, bd) {
                                bot.send_message(mes.chat.id,"Такой человек найден").await?
                            } else {
                                bot.send_message(mes.chat.id,"Такой человек не найден").await?
                            }
                        },
                        Err(_) => bot.send_message(mes.chat.id,"Неверный формат ввода даты").await?,
                    }
                }
            },
            Some(&"/fio") => {
                if m.len() == 0 || m.len() > 3 {
                    bot.send_message(mes.chat.id, "Неверный формат ввода").await?
                } else {
                    let mut fio = m.next().unwrap_or(&"A").to_string().to_uppercase();
                    for part in m {
                        fio = format!("{} {}", fio, part).to_uppercase();
                    };
                    let connection = &mut establish_connection();
                    println!("{}", fio);
                    let fio = format!("{fio}%");
                    if find_fio(connection, fio.clone()) {
                        bot.send_message(mes.chat.id,get_men(connection, fio)).await?
                    } else {
                        bot.send_message(mes.chat.id,"Такие люди не найдены").await?
                    }
                }
            },
            _ => bot.send_message(mes.chat.id, "Неизвестная команда").await?,
        };
        respond(())
    })
    .await;
}
