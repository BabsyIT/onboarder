

use maud::{html, Markup};

pub fn form()-> Markup{
    html! {
        form hx-post="/onboard" { 
            input type="date" name="date" {}
            
            button type="submit" { 
                "Submit" 
            }    
        }
    }
}