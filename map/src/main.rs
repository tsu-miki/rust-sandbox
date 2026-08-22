fn main() {
    let member_names = vec![
        String::from("Sato"),
        String::from("Suzuki"),
        String::from("Takahashi"),
    ];

    let name_lengths: Vec<usize> = member_names
        .iter()
        .map(|member_name| member_name.len())
        .collect();

    println!("{name_lengths:?}");
    println!("{member_names:?}");
}
