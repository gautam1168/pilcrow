import { newApp } from "./buttonwin.pil";

newApp.onButtonPress = () => {
  newApp.setState("pressed");
}

newApp.onButtonRelease = () => {
  newApp.setState("normal");
}

const canvas = document.getElementById("app");
newApp.mount(canvas).then(() => {
  newApp.paint();
});
