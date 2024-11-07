use futures::{join, try_join, TryFutureExt};

pub struct Book {
    title: String,
}

pub struct Music {
    song: String,
}

pub async fn get_book() -> Book {
    Book {
        title: "My favorite book".to_string(),
    }
}

pub async fn get_music() -> Music {
    Music {
        song: "My favorite song".to_string(),
    }
}

/// This is slow since
///  music await will not start until we get the book
pub async fn get_book_music() -> (Book, Music) {
    let book = get_book().await;
    let music = get_music().await;
    (book, music)
}

pub async fn get_book_music_fast() -> (Book, Music) {
    let book = get_book();
    let music = get_music();
    join!(book, music)
}

pub async fn try_get_book() -> Result<Book, ()> {
    let book = Book {
        title: "My favorite book".to_string(),
    };

    println!("thread in book creator: {:#?}", std::thread::current().id());
    println!("created book {}", book.title);

    Ok(book)
}

pub async fn try_get_music() -> Result<Music, String> {
    let music = Music {
        song: "My favorite song".to_string(),
    };

    println!(
        "thread in music creator: {:#?}",
        std::thread::current().id()
    );
    println!("created music {}", music.song);
    Ok(music)
}

pub async fn try_get_book_music() -> Result<(Book, Music), String> {
    let book = try_get_book().map_err(|_| "Unable to get book".to_string());
    let music = try_get_music();

    try_join!(book, music)
}
