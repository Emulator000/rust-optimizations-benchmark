mod json_pull;
mod json_struct;
mod json_value;

pub fn execute() {
    json_pull::execute();
    json_struct::execute();
    json_value::execute();
}
