use chrono::NaiveDateTime;
use maud::{html, Markup};
use rocket::{form::Form, response::content::RawHtml, State};

use crate::persistence::super_babsys::SuperBabsys;

#[derive(FromForm)]
pub struct CurrentDateWithBabsy<'r> {
    //the date they are available from
    date: &'r str,
    // the super babsy id
    id: &'r str,
    // user type
    user_type: &'r str,
}

#[post("/booking", data = "<current_date_with_babsy>")]
pub fn bookings_view_html(
    current_date_with_babsy: Form<CurrentDateWithBabsy<'_>>,
    super_babsys: &State<SuperBabsys>,
) -> RawHtml<String> {
    let from_date =
        NaiveDateTime::parse_from_str(current_date_with_babsy.date, "%Y-%m-%d %H:%M:%S").unwrap();

    let Some(super_babsy) = super_babsys.get_super_babsy(current_date_with_babsy.id) else {
        return RawHtml(
            html! {
                p { "Super Babsy not found" }
            }
            .into_string(),
        );
    };

    let user_type = current_date_with_babsy.user_type;
    let raw = html! {
        div #main-content {
            form hx-post="/booking/new" hx-target="#main-content" hx-swap="outerHTML" {

                input type="hidden" name="super_babsy_id" value=(super_babsy.id);
                input type="hidden" name="date" value=(from_date);
                input type="hidden" name="user_type" value=(user_type);

                ({inputs()})

                    @if user_type == "parent" {
                        ({for_parents()})
                    }

                    @if user_type == "sitter" {
                        ({for_sitter()})
                    }

                button type="submit" { "Submit" };
            }
        }

    }
    .into_string();
    RawHtml(raw)
}

fn inputs() -> Markup {
    html! {
        div {
            label { "Vorname / First name" }
            input type="text" name="first_name";

            label { "Nachname / Last name" }
            input type="text" name="last_name";

            label { "Telefon / Phone" }
            div {
                span { "Switzerland +41" }
                input type="text" name="phone";
            }

            label { "E-Mail / Email" }
            input type="email" name="email";

            label { "Address" }
            div {
                label { "Name der Strasse / Street Name" }
                input type="text" name="street_name";

                label { "Hausnummer / Number" }
                input type="text" name="house_number";

                label { "PLZ / Postal Code" }
                input type="text" name="postal_code";

                label { "Stadt / City" }
                input type="text" name="city";

                label { "Kanton / Canton" }
                input type="text" name="canton";

                label { "Land / Country" }
                input type="text" name="country" value="Switzerland";
            }

            label { "Personalausweis Vorderseite (ID, Reisepass) / Identity Card front (ID, Passport)" }
            input type="file" name="id_front";
            p { "Lade ein Foto der Vorderseite des Personalausweises hoch / Upload a photo of your front of the identity card." }
            p { "ACHTUNG: Die zulässigen Dateierweiterungen sind: jpg, jpeg, png, pdf. Ist dies nicht der Fall, erhalten Sie die folgende Meldung: \"Dateierweiterung ist nicht erlaubt / File's extension is not allowed\"" }

            label { "Personalausweis Rückseite (ID, Reisepass) / Identity Card back (ID, Passport)" }
            input type="file" name="id_back";
            p { "Lade ein Foto der Rückseite des Personalausweises hoch / Upload a photo of the back of your identity card" }
            p { "ACHTUNG: Die zulässigen Dateierweiterungen sind: jpg, jpeg, png, pdf. Ist dies nicht der Fall, erhalten Sie die folgende Meldung: \"Dateierweiterung ist nicht erlaubt / File's extension is not allowed\"" }
        }
    }
}

fn for_sitter() -> Markup {
    html! {
        div {
                   h2 { "Babsy-App: Sitter profile addition" }
                   p { "Dear sitter, welcome to Babsy - the new world of childcare! 😊" }
                   p { "We are thrilled to have you join our fantastic Babsy community." }
                   p { "Trust is very important to us, and in our community, only sitters who have gone through a personal verification process with someone from the Babsy crew are allowed. This ensures that everyone involved can search and find each other in a secure network. But we also need your help for this - in order to complete your sitter profile, we ask you to fill out this questionnaire. That together with the app is then your sitter profile." }
                   p { "What further information can we give you here? Once you've seen the introduction video, there are sure to be a few more questions that we'd be happy to answer before we then conduct our personal conversation on site or via video call (the first conversation) in a further step." }
                   p { "Then we check the documents for completeness and correctness of the information and send you the approval for the app where you can then book directly to finalize the registration." }
                   p { "Later there are always opportunities at our events to talk to us personally, with other parents and with potential sitters." }
                   p { "We have summarized the suggested dates for you at the end of the questionnaire :). We have around 3 \"categories\" of sitters, depending on your needs, a Saturday evening hobby sitter is completely different from the one who takes care of the child when it is sick. Our portfolio of great and competent supervisors is as diverse as the situations are. We recommend sitters to build a portfolio of several families so that you have one or two main families and a few in backup - this way, care is guaranteed at all times. Sitters can get to know the family in advance at any time, if there is enough time to book, we also recommend this. Be it in person or on the virtual path. And should it not be enough, sitters can trust it: Not only the Babsy crew put the families through their paces, but mostly many other Babsy sitters too :) we hope that this will be a little more effective Creates security, should it not be enough to get to know the family beforehand." }
                   p { "And now: see you soon - we look forward to getting to know you! Your Babsy crew" }
                   p { "PS: INFORMATION: PLEASE ANSWER ALL QUESTIONS - FOR INFORMATION TO BE READ, ANSWER YES, OK OR UNDERSTOOD - THANK YOU!" }

                   input type="checkbox" name="intro_read" value="yes" { "I have understood it." }
                   hr {}


                   h3 { "Babsy introduction video" }
                   p { "Please have a look on the video" }
                   a href="https://youtu.be/66RcN0rJ8ZA" { "watch it" }

                   fieldset role="group" {
                           li{
                              input type="radio" name="video_watched" value="yes" { "Yes, I have watched it." }
                           }
                           li{
                               input type="radio" name="video_watched" value="no" { "No, I haven't watched it." }
                           }
                       }
                   hr {}

                   p { "Your Sex (respective your gender)" }
                             fieldset role="group" {
                                 label { input type="radio" name="sex" value="female" {  } "Female" }
                                 label { input type="radio" name="sex" value="male" {  } "Male" }
                                 label { input type="radio" name="sex" value="other" {  } "Other" }
                    }


                    input type="date" name="birth_date" placeholder="Birth Date";

                    label { "Your Civil Status" }
                    select name="civil_status" {
                        option value="single" { "Single" }
                        option value="married" { "Married" }
                        option value="widowed" { "Widowed" }
                        option value="divorced" { "Divorced" }
                        option value="registered_partnership" { "In registered partnership" }
                        option value="dissolved_partnership" { "Dissolved partnership" }
                        option value="unmarried" { "Unmarried (*)" }
                    }

                    p { "The civil status \"Unmarried\" can arise as a consequence of an annulment of the last marriage or as a consequence of a declaration of disappearance of the last spouse." }

                    label{ "Nationality" }
                    input type="text" name="nationality" {}

                    label { "What is your Residency" }
                    p { "For legal reasons, we can only mediate if you are Swiss or have a residence permit B or C. If you don't have this, please message to parent@babsy.ch - there might be solutions." }
                    fieldset role="roup" {
                         legend { "Residency:" }
                         label {
                             input type="radio" name="residency" value="swiss" {  }
                             "Yes, I am Swiss"
                         }

                         label {
                             input type="radio" name="residency" value="permit_b" {  }
                             "Yes, I have a permit B (upload your permit)"
                         }

                         label {
                             input type="radio" name="residency" value="permit_c" {  }
                             "Yes, I have a permit C (upload your permit)"
                         }
                  }

                    label { "Country" }
                    select name="country" {
                        option value="swiss" { "Swiss" }
                        option value="german" { "German" }
                        option value="french" { "French" }
                        option value="other" { "Other" }
                    }

                    hr {}

                    label { "Your Social Contributions Number (OASI) (756.x.x) if not yet mentioned in the App"}
                    input type="text" name="oasi" { }

                    p { "AHV obligation (OASI)" }
                    p { "All sitters who are over than 25 years old must be registered with the AHV" }
                    p { "(comensattion fund) for sitter work. The registration can either be done by us as as Club Babsy or done by the parents itself." }
                    p { "For sitters who are under 25 years old, the status in 2020 is: Up to CHF 750.00 per family per year can be earned without registering for OASI." }

                    fieldset role="group" {
                        label { input type="radio" name="ahv" value="no" {  } "Yes, I am under 25 and it does not include an AHV registration up to CHF 750.00 per family" }
                        label { input type="radio" name="ahv" value="yes" {  } "Yes I am over 25 and therefore I need an AHV registration when I work" }
                    }
                     hr {}

                    input type="text" name="from_where_did_you_know" { "
                        I got to know about Babsy via…
                        (Please let us know if someone recommended us that we can say thank you to the person given name and family name)
                        " 
                    }
                    hr {}


                  selecion{
                      label { "Do you have references? Please state with first name, surname, adress, telephonenumber and E-Mail of the reference, please inform people in advance. Please send more references to sitter@babsy.ch" }
                      option value="yes" { "Yes" }
                      option value="no" { "No" }
                  }


                  label { "Profile Picture" }
                  input type="file" name="profile_picture" {  };


                  hr {}
                  label { "Do you have a SRK babysitter certificate (from the Swiss Red Cross)." }
                              fieldset role="group" {
                                  label { input type="radio" name="srk_certificate" value="yes" {  } "Yes" }
                                  label { input type="radio" name="srk_certificate" value="no" {  } "No" }
                                  label { input type="radio" name="srk_certificate" value="other" {  } "Other" }
                              }

                              label { "Did you do a first aid course (e.g. within doing your driving license)." }
                              fieldset role="group" {
                                  label { input type="radio" name="first_aid_course" value="yes" {  } "Yes" }
                                  label { input type="radio" name="first_aid_course" value="no" {  } "No" }
                                  label { input type="radio" name="first_aid_course" value="other" {  } "Other" }
                              }

                              label { "Please watch this video, you have to know this life-saving measure." }
                              fieldset role="group" {
                                  label { input type="radio" name="life_saving_video" value="yes" {  } "Yes, I have watched it." }
                                  label { input type="radio" name="life_saving_video" value="no" {  } "No, I did not watch it." }
                              }

                              hr {}

                              label { "I know the important emergency numbers." }
                              p { "117 Police" }
                              p { "118 Firefighters" }
                              p { "144 Ambulance resp. Rescue" }
                              p { "1414 Swiss rescue" }
                              p { "112 European emergency number" }
                              fieldset role="group" {
                                  label { input type="radio" name="emergency_numbers" value="yes" {  } "Yes, I know the numbers by heart." }
                                  label { input type="radio" name="emergency_numbers" value="no" {  } "No, I do not know the numbers." }
                              }

                              label { "We ask you to inform yourself about \"Sudden infant death syndrome\". Also in our courses we do care about this topic. Click here" }
                              fieldset role="group" {
                                  label { input type="radio" name="sids_info" value="yes" {  } "Yes, I informed myself about it." }
                                  label { input type="radio" name="sids_info" value="no" {  } "No, I have not informed myself about it." }
                              }

                              label { "We ask you to inform yourself about \"Shaken baby syndrome\". Also in our courses we do care about this topic. Click here" }
                              fieldset role="group" {
                                  label { input type="radio" name="sbs_info" value="yes" {  } "Yes, I informed myself about it." }
                                  label { input type="radio" name="sbs_info" value="no" {  } "No, I have not informed myself about it." }
                              }

                              hr {}

                              label { "Upload your criminal record. If you do not have one, make a new one to send it" }
                              input type="file" name="criminal_record";
                              p { "Also, we will order the special criminal record for your which is absolutely necessary. The special criminal record is including sexual offenses."}
                              p { "The instructions are included in the email attachment, please send them to us at sitter@babsy.ch within 3 months. "}

                              p { "We set a sitter profile online, where you can check your information and which the parents can also see. Parents will not see any private details.
                              The parents can choose you directly as a sitter and have to contact you via the App-Chatt. It is your honorable duty as Babsy-Sitter to inform us of the hours you have spent with a family once a month by the 24th of a month to
                              Click here to view the form
                              So that our sitters are secured, we do our best and organize insurance for you and clarify all important things with the authorities, so that everything is legally compliant and we can only give you certified families. We help you wherever we can and need your support. If parents ask you directly, please let us know. We do everything on a voluntary basis and also set up a reward system that benefits you, but this will only work if you are honest with us and we know every hour. THANK YOU for your help." }

                              select {
                                  option value="yes" { " Yes, I understand this system and send my lessons to Babsy at the 24th of every month where I worked: sitter@babsy.ch" }
                                  option value="no" { "No, I did not understand the system and get in touch with Babsy via email to sitter@babsy.ch or by phone" }
                              }

                              hr {}

                              p { "Information net-rate:
                              What is your net hourly wage? The net wage is the one that should be paid to you. Since Babsy parents normally take over the entire wage deductions, you should give us the highest possible tariff, which is negotiable downwards in order to accommodate the parents so that you take over all contributions, which is not a matter of course." }

                              label { "Net rate" }
                              input type="text" name="net_rate" { }

                              input type="text" name="bank" placeholder="Bank name" {}
                              input type="text" name="bank_address" placeholder="Bank adress" {}
                              input type="text" name="IBAN" placeholder="IBAN" {}

                            input type="checkbox" name="no_photo_understood" {
                                "Important: During your jobs photos of children or the personal environment are forbidden to be taken during an operation. If parents so wish, images must be deleted immediately after use."
                            }

                            hr{}

                            a href="https://babsy.ch/wp-content/uploads/2022/09/01_AGB_DE_20220101.pdf" { "TAC" }
                            input type="checkbox" name="tca" {
                              "I agree to the TAC as well as all the legal conditions. "
                            }

                            p { "Here are the App-Links:" }
                            ul{
                                  li{ a href="https://apps.apple.com" { "Click here for Apple" } }
                                  li{ a href="https://play.google.com" { "Click here for Android" } }
                            }

                            label { "Notes" }
                            textarea name="notes" { "" }

        }


    }
}

fn for_parents() -> Markup {
    html! {
        div {
            h2 { "Babsy-App: Family profile addition" }
            p { "Dear family Welcome to Babsy - the new world of childcare! 😊" }
            p { "Nice, did you find your way to us and are you interested in our fantastic Babsy community." }
            p { "Trust is very important to us, in our community there are only parents and sitters who have all gone through a personal verification process with someone from the Babsy crew. In this way we ensure that everyone involved can search and find each other in a secure network. But we also need your help for this - in order to complete your family profile, we ask you to fill out this questionnaire. That together with the app is then your family profile." }
            p { "What further information can we give you here? Once you've seen the introduction video, there are sure to be a few more questions that we'd be happy to answer before we then conduct our personal conversation on site or via video call (the first conversation) in a further step." }
            p { "Then we check the documents for completeness and correctness of the information and send you the approval for the app where you can then book directly to finalize the registration." }
            p { "Later there are always opportunities at our events to talk to us personally, with other parents and with potential sitters." }
            p { "We have summarized the suggested dates for you at the end of the questionnaire :). We have around 3 \"categories\" of sitters, depending on your needs, a Saturday evening hobby sitter is completely different from the one who takes care of the child when it is sick. Our portfolio of great and competent supervisors is as diverse as the situations are. We recommend parents to build a portfolio of several sitters so that you have one or two main sitters and a few in backup - this way, care is guaranteed at all times. Parents can get to know the sitter in advance at any time, if there is enough time to book, we also recommend this. Be it in person or on the virtual path. And should it not be enough, parents can trust it: Not only the Babsy crew put the sitters through their paces, but mostly many other Babsy parents too :) we hope that this will be a little more effective Creates security, should it not be enough to get to know the sitter beforehand." }
            p { "And now: see you soon - we look forward to getting to know you! Your Babsy crew" }
            p { "PS: INFORMATION: PLEASE ANSWER ALL QUESTIONS - FOR INFORMATION TO BE READ, ANSWER YES, OK OR UNDERSTOOD - THANK YOU!" }
            label { "Understood - Thank you" }
            input type="text" name="understood" placeholder="Understood - Thank you";


            h3 { "Babsy introduction video" }
            p { "Please have a look on the video (watch it)" }
            a href="https://youtu.be/66RcN0rJ8ZA" { "https://youtu.be/66RcN0rJ8ZA" }

            fieldset role="group" {
                    li{
                       input type="radio" name="video_watched" value="yes" { "Yes, I have looked at it." }
                    }
                    li{
                        input type="radio" name="video_watched" value="no" { "No, I haven't looked at it." }
                    }
            }

            label { "Your Civil Status" }
            select name="civil_status" {
                option value="single" { "Single" }
                option value="married" { "Married" }
                option value="widowed" { "Widowed" }
                option value="divorced" { "Divorced" }
                option value="registered_partnership" { "In registered partnership" }
                option value="dissolved_partnership" { "Dissolved partnership" }
                option value="unmarried" { "Unmarried (*)" }
            }
            p { "The civil status \"Unmarried\" can arise as a consequence of an annulment of the last marriage or as a consequence of a declaration of disappearance of the last spouse." }

            label { "What is your Nationality" }
            p { "For legal reasons, we can only mediate if you are Swiss or have a residence permit B or C. If you don't have this, please message to parent@babsy.ch - there might be solutions." }
            fieldset role="roup" {
                 legend { "Language preference:" }
                 label {
                     input type="radio" name="nationality" value="swiss" {  }
                     "Yes, I am Swiss"
                 }
                 label {
                     input type="radio" name="nationality" value="permit_b" {  }
                     "Yes, I have a permit B"
                 }
                 label {
                     input type="radio" name="nationality" value="permit_c" {  }
                     "Yes, I have a permit C"
                 }
         }


            label { "I got to know about Babsy via…" }
            input type="text" name="referral" placeholder="Enter your answer";

            label { "Why do we ask if your child/ren have any allergies or special needs (sickness etc.) and who can see this?" }
            p { "Explanation: For trainings etc. it is important that we sitters can sensitize them to many topics and also train in our trainings." }
            input type="text" name="special_needs" placeholder="Special needs (sickness etc.)";

             fieldset role="roup" {
                 legend { "Are you a single parent?" }
                 p { "If so, get in touch with us, we can also offer help here and know institutions that provide support here." }
                 label { input type="radio" name="single_parent" value="yes" {  } "Yes" }
                 label { input type="radio" name="single_parent" value="no" {  }  "No" }
             }


            label { "Please upload a small family photo for your future profile (you have the chance to make pictures via Babsy e.g. at the \"meet & greet Events. If there are concerns, the children's face may e.g. be covered with a smiley. We are happy to let the sitters know who is the family, they are caring for." }
            input type="file" name="family_photo";
            p { "If you have it handy, upload it; if not, send it to the email address provided parent@babsy.ch." }
            p { "ACHTUNG: Die zulässigen Dateierweiterungen sind: jpg, jpeg, png, pdf. Ist dies nicht der Fall, erhalten Sie die folgende Meldung: \"Dateierweiterung ist nicht erlaubt / File's extension is not allowed\"" }

            label { "Accident insurance for domestic workers is, vor non-club-members, compulsory, for employees who are subject to AHV and can be taken out from any Swiss insurer for around CHF 100.00 per year." }

            fieldset role="group" {
               legend { "Do you have an accident insurance for housekeeping-staff?" }
               label{ input type="radio" name="accident_insurance" value="yes" {  } "Yes" }
               label{ input type="radio" name="accident_insurance" value="no" {  } "No" }
               label{ input type="radio" name="accident_insurance" value="others" {  } "Others" }
            }


            p { "Attention, in the basic offers of \"Babsy-Budget\" and \"Babsy-trustee\", the parents are always the employers of the childcare workers and the provisions of the OR apply." }
            p { "Information tariffs/hourly wages" }
            p { "The sitters generally state the net wage. The net wage is the one that should be paid to them. Since Babsy parents usually take over the entire wage deductions in the easy process with \"Babsy-trustee\", the tariff in the sitter profiles (if OASI-duty additional the OASI-price) is the maximum that you can expect. The sitters are ready to negotiate and are happy to meet your desires, especially for long-term assignments." }


            p { "We will inform you as soon as this is the case. If you would like to join the association anyway and would like to support us in this way, please return the signed membership form to us." }
            fieldset role="group" {
                legend { "I agree to the TAC as well as all the legal conditions" }
                label { input type="radio" name="tac_agreement" value="yes" {} "Yes, I read them, and I agree" }
                label { input type="radio" name="tac_agreement" value="no" {}  "No, I do not agree" }
                label { input type="radio" name="tac_agreement" value="wait" {} "I want to wait after the personal talk to decide this" }
            }


            p { "Here are the App-Links:" }
            ul{
                  li{ a href="https://apps.apple.com" { "Click here for Apple" } }
                  li{ a href="https://play.google.com" { "Click here for Android" } }
            }

            label { "Hinweise / Notes" }
            textarea name="notes" { "" }
        }
    }
}
