# BaselHack 2024 - Babsy - onboarder

## Description

This app will help to not loose sitters or parents when they sign up in the mobile app.

## Actors

### SuperBabsy

A SuberBabsy is a person who is responsible for the onboarding of new sitters and parents.

### Sitter
A sitter is a person who is looking for a job as a babysitter.

### Parent
A parent is a person who is looking for one or more babysitters.

### Admin
An admin might want to see the statistics of the onboarding process.


## Conceptual Problem

### Process: 
1. In the mobile app users (parent and sitters) can sign up. 
1. The user has to fill out a small part of the profile and then the user is added but the account is not yet confirmed.
1. In order to confirm the account the user has to click on a link in the app which takes them to the main website.
1. In the website the user has to fill out a form with more details and what SuperBabsy they want with which availability for a video call.
1. The user will submit their video call booking request.
1. The SuperBabsy will see all the bookings and then accept the booking they if they find the user suitable.
1. The SuperBabsy will have a video call with the user to get to know them.
1.  If the SuperBabsy finds the user suitable they will confirm the account.

### Issues:
1. When switching from app to the website many users will not complete the registration process:
- because they do not understand where to continue.
- they are annoyed becaue the website is not responsive to phones.
- they think the sign up does not work because the signup form needs 4-5 seconds to load.

2. When filling out the form the will not complete the form because:
- When choosing a date they can choose a SuperBabsy who is not available.
- It is not clear that not all SuperBabsy are available for what language for which type of user e.g. not all SuperBabsys can take Sitter with the language English.
- They have to put in a lot of information which they already provided in the App.

## Technical Problems

### Issuses:

The application has a diffrent backend then the website.
The website uses a plugin called "bookly" which persists data in its own way.
The plugin "bookly" is a risk, wordpress plugins can be removed from the owner. The fallback is manual calls and emails.
Because of the way bookly uses the database and in memory models the wordpress website can not be scaled horizontally.


## Solution

### App
As the app knows if the user is a Sitter or a Parent we can use this information to fill into link.

> https://onboarding.babsy.com/?user_type=parent

### Onboarder

The onboarder can read the url and show only the SuperBabsys to the user who are capable of taking the potenial voice call and onboarding.
The onboarder gives the user simplified options to choose their availability and wished language, which will reduce the possible SuperBabsys to choose from.
The onboarder may prefill the form with the information the user already provided in the app.

