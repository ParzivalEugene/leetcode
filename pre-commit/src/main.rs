mod constants;
mod data;
mod enums;
mod services;

fn main() {
    let records = services::readme::read_table();
    let solutions = services::solutions::get_solutions();

    let updated_solutions = services::solutions::update_solutions(records.clone(), solutions);
    let new_table = services::readme::update_table(updated_solutions);
    let new_counters = services::readme::update_counters(&records);

    services::readme::write(new_table, new_counters);
}
