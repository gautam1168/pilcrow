import { App, paint } from "./pilcrow";

App.onButtonPress = () => {
  App.item.state = "pressed";
}

App.onButtonRelease = () => {
  App.item.state = "normal";
}

const canvas = document.getElementById("app") as HTMLCanvasElement;
App.mount(canvas).then(() => {
  paint(App);
});
