fn main() {
    println!(
        "{}",
        users::get_user_by_uid(users::get_effective_uid())
            .unwrap()
            .name()
            .to_str()
            .unwrap()
    );
}
