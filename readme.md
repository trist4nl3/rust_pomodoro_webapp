# Pomodoro Web App

This is a Pomodoro web app built using Rust and the Yew framework. It aims to help users manage their time efficently by following the time management method developed by Francesco Cirillo.

## Demo

- You can try the Pomodor web app [here](https://trist4nl3.github.io/rust_pomodoro_webapp/)

## Features
- Timer for Pomodoro sessions
- Customizable session durations (Pomodoro work time, Break time)
- Start, pause, resume, and reset functionality
- Simple user interface
- Anime inspirations

## Planned Features
 [ ] Ability to switch from default settings from Animedoro to Pomodoro
 [ ] Change background image
 [ ] Add visual and audio cues for changes
 [ ] Ability to save settings locally using cookies
 [ ] OAuth login system for backend
 [ ] Add backend for leaderboard system

## Installation
To run this project locally,
1. Clone the repository and build the project using Rust's package manager Cargo:
```
git clone https://github.com/trist4nl3/rust-pomodoro-webapp.git
cd rust-pomodoro-timer
cargo build --release
```
2. Compose the docker container
```
cd devcontainer
docker-compose up
```
3. Run the timer
```
trunk serve --open
```
4. Open in localhost
```
localhost:8080
```

## Contributing
Feel free to contribute! This project is going to remain open source.

## License

This project is licensed under the MIT License

## Acknowledgements
- Yew - Rust framework for building client web apps
- Pomodoro Technique - Time management method -> 25/5 Work Split
- Animedoro Technique - 50 minutes of study -> 1 Anime Episode
