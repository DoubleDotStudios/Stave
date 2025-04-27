#[derive(Default)]
struct BuildConfig {
    user: User,
}

#[derive(Default)]
struct User {
    username: String,
    password: String,
}
