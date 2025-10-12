import { useEffect, useMemo, useState } from "react";
import Particles, { initParticlesEngine } from "@tsparticles/react";
import {
  type Container,
  type ISourceOptions,
  MoveDirection,
  OutMode,
} from "@tsparticles/engine";
// import { loadAll } from "@tsparticles/all"; // if you are going to use `loadAll`, install the "@tsparticles/all" package too.
// import { loadFull } from "tsparticles"; // if you are going to use `loadFull`, install the "tsparticles" package too.
import { loadSlim } from "@tsparticles/slim"; // if you are going to use `loadSlim`, install the "@tsparticles/slim" package too.
// import { loadBasic } from "@tsparticles/basic"; // if you are going to use `loadBasic`, install the "@tsparticles/basic" package too.

const ParticlesBackground = () => {
  const [init, setInit] = useState(false);

  // this should be run only once per application lifetime
  useEffect(() => {
    initParticlesEngine(async (engine) => {
      // you can initiate the tsParticles instance (engine) here, adding custom shapes or presets
      // this loads the tsparticles package bundle, it's the easiest method for getting everything ready
      // starting from v2 you can add only the features you need reducing the bundle size
      //await loadAll(engine);
      //await loadFull(engine);
      await loadSlim(engine);
      //await loadBasic(engine);
    }).then(() => {
      setInit(true);
    });
  }, []);

  const particlesLoaded = async (container?: Container): Promise<void> => {
    console.log(container);
  };

  const options: ISourceOptions = useMemo(
    () => ({
      autoPlay: true,
      background: {
        color: {
          value: "#000000"
        },
        opacity: 1
      },
      fullScreen: {
        enable: true,
        zIndex: 0
      },
      detectRetina: true,
      fpsLimit: 120,
      interactivity: {
        detectsOn: "window",
        events: {
          onClick: {
            enable: false,
            mode: "repulse"
          },
          onHover: {
            enable: false,
            mode: "bubble",
            parallax: {
              enable: false,
              force: 2,
              smooth: 10
            }
          },
          resize: {
            delay: 0.5,
            enable: true
          }
        }
      },
      particles: {
        color: {
          value: "#ffffff"
        },
        move: {
          angle: {
            offset: 0,
            value: 90
          },
          center: {
            x: 50,
            y: 50,
            mode: "percent",
            radius: 0
          },
          direction: "none",
          drift: 0,
          enable: true,
          random: false,
          size: false,
          speed: {
            min: 0.1,
            max: 1
          },
          straight: false,
          outModes: {
            default: "out",
            bottom: "out",
            left: "out",
            right: "out",
            top: "out"
          }
        },
        number: {
          density: {
            enable: true,
            width: 1920,
            height: 1080
          },
          limit: {
            mode: "delete",
            value: 0
          },
          value: 160
        },
        opacity: {
          value: {
            min: 0.1,
            max: 1
          },
          animation: {
            count: 0,
            enable: true,
            speed: 1,
            decay: 0,
            delay: 0,
            sync: false,
            mode: "auto",
            startValue: "random",
            destroy: "none"
          }
        },
        shape: {
          close: true,
          fill: true,
          type: "circle"
        },
        size: {
          value: {
            min: 1,
            max: 3
          },
          animation: {
            count: 0,
            enable: false,
            speed: 5,
            decay: 0,
            delay: 0,
            sync: false,
            mode: "auto",
            startValue: "random",
            destroy: "none"
          }
        },
        links: {
          enable: false
        },
        life: {
          count: 0,
          delay: {
            value: 0,
            sync: false
          },
          duration: {
            value: 0,
            sync: false
          }
        },
        roll: {
          darken: {
            enable: false,
            value: 0
          },
          enable: false,
          enlighten: {
            enable: false,
            value: 0
          },
          mode: "vertical",
          speed: 25
        },
        rotate: {
          value: 0,
          animation: {
            enable: false,
            speed: 0,
            decay: 0,
            sync: false
          },
          direction: "clockwise",
          path: false
        }
      }
    }),
    [],
  );

  if (init) {
    return (
      <Particles
        id="tsparticles"
        particlesLoaded={particlesLoaded}
        options={options}
      />
    );
  }

  return <></>;
};

export default ParticlesBackground;