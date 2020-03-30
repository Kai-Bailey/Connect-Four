# Get Started
Build the Frontend
```bash
cargo install -f cargo-web
rustup default nightly
```
Go to the connect-four-frontend directory, then run: 
```bash
bash build.sh
```
Install and start MongoDB: https://www.mongodb.com/download-center/community <br/>
Go to the connect-four directory, then run: 
```bash
cargo run
```
Open the game at http://localhost:8000/


# Design Document
## Major Innovations
## Design Decisions
### Rocket
[Rocket](https://rocket.rs/) is a web framework for Rust. Rocket routes incoming requsts to their approriate handlers. We choice Rocket because of its great developer support, simplicity and support for JSON.



## FAQ
### What can we do on a computer than we can’t do on a printed board?
Computer games have two main advantages over their physical counter part. The first is the ability to play against other players located anywhere in the world. The second advantage online games have is economy of scale. Once an online game is built each addition additional new players cost pennies in server fees. In comparison, everytime a new board game is purchased a physical board and pieces must be manufactured creating a significant cost per new user. Online games scale much better than physical games.
### What is a computerized opponent? What are its objectives?
A computerized opponent allows the computer to play against a human. The objective of the computer player is not to beat the human evertime, but instead provide an enjoyable experience. This may mean allowing the human to scale the difficulty to a point where the computer opponent is challenging but still beatable.
### What design choices exist for the Interface components? 
The core design principles for this project are simplicity, and fun. Simplicity means the game/interface should be intuitive to play requiring little to no written instructions. A drawer on the left hand of the screen was chosen to clearly lay out each of the game options. The game control of clicking on the row to drop a token was chosen because it is the most intuitive way to interact with the online board.
Fun means the the game/interface should understand that this web app was built for enjoyment purposes simply from the color scheme and fonts. For this reason a light orange was chosen as the main color scheme.
### What does exception handling mean in a GUI system?
All exceptions that are not critical (would crash the application) or related to a users behaviour should be hidden from the user. This includes things such as warnings. Exceptions that arise from user behaviour should display an informative message to the user. Examples include the user not having an internet connection or inputting invalid values. Critical errors should explain to the user that something went wrong with the application without discolsing specifics (for security reasons).
### Do we require a command-line interface for debugging purposes?
Yes a command-line interface is critical for debugging purposes. It allows developers to manually test the code without having to go through all the steps in the GUI. Developers can quickly interact with the application from their terminal to narrow down the source of a bug.
### What are Model-View-Controller and Model-View-Viewmodel? Are either applicable to your design?
Model-view-controller and model-view-viewmodel are common programming paradigms used to structure and seperate different parts of the code. Model-view-controller seperates the code into the model which contains the data, a view which is displayed to the user and the controller which is the buisness logic that maps between the view and the data. In our application the view is implemented in html, the model is the mongodb database and controller is implemented in Rust.
## Known Errors
