import * as pilcrow from "pilcrow";

const el = document.getElementById("canvas");
if (el) {
  pilcrow.drawSmiley();
} else {
  pilcrow.greet();
}

// pilcrow.run();

// import ui from "./application.pil"; // <-- a kind of binary file to discourage direct editing

// let controller = pilcrow.generateBindings(ui);
// /**
//  * 
//   controller = {
//     screen1: {
//       onClick: () => {},
//       onLoad: () => {},
//       onUnload: () => {},
//       subScreen: () => {}
//     },
//     screen2: {
//       title: "Some title",
//       onClick: () => {},
//       beforeLoad: () => {}
//     }
//   }
//  */

// controller.screen1.title = "Some page title";

// controller.screen1.onButtonPress = () => {
//   controller.screen1.button.state = "pressed";
// }

// controller.screen1.onButtonUp = () => {
//   controller.screen1.button.state = "normal";
// }

// pilcrow.startApp(ui, controller)
