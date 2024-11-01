

use chrono::Utc;
use maud::{html, Markup};
use rocket::time::Instant;

pub fn form()-> Markup{
    let dt = Utc::now();
       
       // Format the datetime to the required string format
       let formatted_time = dt.format("%Y-%m-%d").to_string();
    html! {
        form hx-post="/onboard" { 
            input type="date" name="date" min={(formatted_time)} {
                
            }
           
            
            button type="submit" { 
                "Submit" 
            }    
        }
    }
}