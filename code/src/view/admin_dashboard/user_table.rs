use maud::{html, Markup};

use crate::superbabsys::SuperBabsy;

pub fn generate(babsys: Vec<SuperBabsy>) -> Markup {
    html! {
        div ."table-container" {
            table ."overflow-auto" {
                thead {
                    tr {
                        th { "ID" }
                        th { "Name" }
                        th { "Description" }
                        th { "Parent Compentencies" }
                        th { "Sitter Compentencies" }
                        th { "Image Url" }
                        th { "Availability" }
                    }
                }
                tbody {
                    @for babsy in babsys {
                        tr {
                            td { (babsy.get_id()) }
                            td { (babsy.get_name()) }
                            td { (babsy.get_description()) }
                            td { (babsy.parent_comp_as_string()) }
                            td { (babsy.sitter_comp_as_string()) }
                            td { (babsy.get_image_url_string_or_none()) }
                            td {
                                div {
                                    details{
                                        summary { "available hours: " }
                                        {(hours(babsy))}
                                        "..."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn hours(babsy: SuperBabsy) -> Markup {
    let babsy = babsy
        .get_available_dates_from_first()
        .into_iter()
        .take(20)
        .map(|hour| hour.to_string());

    html! {
        ul{
            @for hour in babsy {
                li{
                    p .hour { (hour) }
                }
            }

        }

    }
}
