const MAX_APP_NUM: usize = 16;
struct Appmanager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}
