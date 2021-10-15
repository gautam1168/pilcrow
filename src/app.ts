// import { App, paint } from "./pilcrow";
import { newApp, paint } from "./buttonwin.pil";

// console.log("imported: ", newApp);
newApp.onButtonPress = () => {
  newApp.setState("pressed");
  // App.item.state = "pressed";
}

newApp.onButtonRelease = () => {
  newApp.setState("normal");
  // App.item.state = "normal";
}

const canvas = document.getElementById("app") as HTMLCanvasElement;
newApp.mount(canvas).then(() => {
  paint(newApp);
});

// import parser from "./parser";
// console.log(
// parser.parse(`Item { id: myButton_Symbol }`)
// );
