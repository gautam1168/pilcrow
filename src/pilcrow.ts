type State = Record<string, {
  url: string, 
  ref?: HTMLImageElement
}>;

interface Item {
  state: "pressed" | "normal",
  states: State
}

interface App {
  onButtonPress?: () => void,
  onButtonRelease?: () => void,
  item: Item,
  canvas?: HTMLCanvasElement,
  context?: CanvasRenderingContext2D,
  mount: (canvas: HTMLCanvasElement) => Promise<void>
};

export const App: App = {
  item: new Proxy({
    state: "normal",
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
      if (this.onButtonPress) {
        this.onButtonPress();
      }
    });

    canvas.addEventListener("mouseup", () => {
      if (this.onButtonRelease) {
        this.onButtonRelease();
      }
    });

    return Promise.all(states.map(it => it.downloaded)).then(() => {});
  }
};

export function paint(app: App) {
  const item = app.item;
  const context = app.context;
  const image = item.states[item.state].ref;
  if (image && context) {
    context.drawImage(image, 0, 0);
  }
}