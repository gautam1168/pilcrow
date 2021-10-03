// const canvas = document.getElementById("app") as HTMLCanvasElement;

// const App = {
//   mount(canvas: HTMLCanvasElement) {
//     const ctx = canvas.getContext("2d");
//     // ctx?.drawImage();
//   }
// };

// App.mount(canvas);
type State = Record<string, {
  url: string, 
  ref?: HTMLImageElement
}>;

interface Item {
  state: "pressed" | "normal",
  states: State
}

interface App {
  onButtonClick?: () => void,
  item: Item,
  canvas?: HTMLCanvasElement,
  context?: CanvasRenderingContext2D,
  mount: (canvas: HTMLCanvasElement) => Promise<void>
};

const App: App = {
  item: new Proxy({
    state: "pressed",
    states: {
      "pressed": {
        url: "/public/pressed.png"
      },
      "normal": {
        url: "/public/normal.png"
      }
    }
  }, {
    set(target: Item, prop: string, value: keyof Pick<Item, "states">) {
      (target as any)[prop] = value;
      if (prop === "state") {
        paint(App);
      }
      return true;
    }
  }),
  mount(canvas) {
    this.canvas = canvas;
    const context = canvas.getContext("2d");
    if (context) {
      this.context = context;
    }
    let states = Object.keys(this.item.states).map(key => {
      let item = this.item.states[key];
      let image = new Image();
      image.src = item.url;
      return {
        key,
        url: item.url,
        ref: image,
        downloaded: new Promise((resolve, reject) => {
          image.onload = resolve;
          image.onerror = reject;
          image.onabort = reject;
        })
      };
    });

    this.item.states = states.reduce((accum, state) => {
      accum[state.key] = { url: state.url, ref: state.ref };
      return accum;
    }, {} as State);

    canvas.addEventListener("mousedown", () => {
      this.item.state = "pressed";
    });

    canvas.addEventListener("mouseup", () => {
      this.item.state = "normal";
    });

    return Promise.all(states.map(it => it.downloaded)).then(() => {});
  }
};

function loadImages(item: Item) {

}

function paint(app: App) {
  const item = app.item;
  const context = app.context;
  const image = item.states[item.state].ref;
  if (image && context) {
    context.drawImage(image, 0, 0);
  }
}

// import App from "./app.pil";
App.onButtonClick = () => {
  console.log("Clicked");
}

const canvas = document.getElementById("app") as HTMLCanvasElement;
App.mount(canvas).then(() => {
  paint(App);
});