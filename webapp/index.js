import * as pilcrow from "pilcrow";

// pilcrow.greet();

const el = document.getElementById("canvas");
if (el) {
  pilcrow.drawSmiley();
} else {
  pilcrow.greet();
}