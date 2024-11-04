# BaselHack 2024 - Babsy - Onboarder

In this README, you will find details on how the challenge played out.
For information on the code, please refer to [code/README.md](code/README.md).

## Description

This app is designed to help prevent the loss of sitters or parents during the mobile app signup process.

## Actors

### SuperBabsy

A SuperBabsy is responsible for onboarding new sitters and parents.

### Sitter
A sitter is a person looking for a job as a babysitter.

### Parent
A parent is a person seeking one or more babysitters.

### Admin
An admin may want to view statistics related to the onboarding process.

## Conceptual Problem

### Process: 
1. In the mobile app, users (parents and sitters) can sign up.
2. The user fills out a basic profile section, and their account is added but not yet confirmed.
3. To confirm the account, the user clicks a link in the app, taking them to the main website.
4. On the website, the user fills out a form with additional details, including their preferred SuperBabsy and availability for a video call.
5. The user submits a video call booking request.
6. SuperBabsy reviews all bookings and, if they find the user suitable, accepts the booking.
7. SuperBabsy conducts a video call with the user to get to know them.
8. If SuperBabsy finds the user suitable, they confirm the account.

### Issues:
1. When switching from the app to the website, many users do not complete the registration process because:
   - they do not understand where to continue,
   - they are frustrated because the website is not responsive on phones,
   - they think the signup process is broken due to a 4-5 second delay in loading the signup form.

2. While filling out the form, users may abandon it because:
   - they can select a SuperBabsy who is unavailable,
   - it’s unclear that not all SuperBabsys are available for all languages or types of users (e.g., not all SuperBabsys can onboard English-speaking sitters),
   - they have to re-enter information already provided in the app.

## Technical Problems

### Issues:

- The application backend is separate from the website backend.
- The website uses a plugin called "Bookly" that stores data independently.
- The "Bookly" plugin poses a risk because WordPress plugins can be discontinued by their owners. The fallback solution is to manually schedule calls and emails.
- Due to Bookly’s database and in-memory model, the WordPress website cannot scale horizontally.

## Solution

### App
Since the app knows if the user is a Sitter or a Parent, we can use this information in the link:

> https://onboarding.babsy.com/?user_type=parent

### Onboarder

The onboarder can read the URL and display only the SuperBabsys who are available for the potential video call and onboarding.
The onboarder offers simplified options for the user to choose their availability and preferred language, reducing the selection of SuperBabsys to only those who are available.
The onboarder may also prefill the form with information the user already provided in the app.
