use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = Post::new();

    post.add_text("Today i had bacon");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text(", and it was great!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("Today i had bacon, and it was great!", post.content());

    let mut post = Post::new();

    post.add_text("More bacon today, i need to start watching my weight");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("This text should not be added");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("This text should not be added");
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "More bacon today, i need to start watching my weight",
        post.content()
    );
}
