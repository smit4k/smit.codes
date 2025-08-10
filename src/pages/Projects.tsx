import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import { FaGithub, FaGlobe, FaRust, FaReact, FaJava } from "react-icons/fa"; // added FaJava
import { RiTailwindCssFill } from "react-icons/ri";
import { SiModrinth } from "react-icons/si";

const Projects = () => {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="bg-card border border-border rounded-lg p-8 max-w-2xl w-full">
        <div className="flex justify-between items-start mb-8">
          <img
            src="/smit_pfp.svg"
            alt="Smit's profile picture"
            className="w-10 h-10 rounded-full object-cover border-2 border-border"
          />
          <Navigation />
        </div>
        <div className="space-y-8">
          <div>
            <h2 className="text-xl font-semibold mb-4 text-foreground">Projects</h2>
            <p className="text-muted-foreground mb-6">
              Most of my projects are open source and available on my Github,{" "}
              <a
                href="https://github.com/smit4k"
                target="_blank"
                rel="noopener noreferrer"
                className="text-link hover:text-link-hover transition-colors"
              >
                @smit4k
              </a>
              . Here are some highlights of my work:
            </p>
            <div className="space-y-4">
              {/* Conversia */}
              <div className="border border-border rounded-lg p-4 relative">
                <a
                  href="https://github.com/smit4k/conversia"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="absolute top-4 right-4 text-muted-foreground hover:text-link transition-colors"
                  aria-label="View conversia on GitHub"
                >
                  <FaGithub size={18} />
                </a>
                <div className="pr-16">
                  <h3 className="font-medium text-foreground mb-2">conversia</h3>
                  <p className="text-sm text-muted-foreground mb-3">
                    A powerful, multi-purpose file utility bot using the serenity and poise frameworks
                  </p>
                  <div className="flex gap-2">
                    <span className="text-xs px-2 py-1 bg-[hsl(var(--rust))] text-secondary-foreground rounded"><FaRust /></span>
                  </div>
                </div>
              </div>
              {/* smit.codes */}
              <div className="border border-border rounded-lg p-4 relative">
                <div className="absolute top-4 right-4 flex gap-2">
                  <a
                    href="https://smit.codes"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-muted-foreground hover:text-link transition-colors"
                    aria-label="View smit.codes site"
                  >
                    <FaGlobe size={18} />
                  </a>
                  <a
                    href="https://github.com/smit4k/smit.codes"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-muted-foreground hover:text-link transition-colors"
                    aria-label="View smit.codes on GitHub"
                  >
                    <FaGithub size={18} />
                  </a>
                </div>
                <div className="pr-16">
                  <h3 className="font-medium text-foreground mb-2">smit.codes</h3>
                  <p className="text-sm text-muted-foreground mb-3">
                    This website, a personal homepage, and my first time using React and TailwindCSS
                  </p>
                  <div className="flex gap-2">
                    <span className="text-xs px-2 py-1 bg-[hsl(var(--lang-react))] text-[hsl(var(--lang-react-foreground))] rounded"><FaReact /></span>
                    <span className="text-xs px-2 py-1 bg-[hsl(var(--lang-tailwind))] text-[hsl(var(--lang-tailwind-foreground))] rounded"><RiTailwindCssFill /></span>
                  </div>
                </div>
              </div>
              {/* Quicknote */}
              <div className="border border-border rounded-lg p-4 relative">
                <div className="absolute top-4 right-4 flex gap-2">
                  <a
                    href="https://modrinth.com/mod/quicknote"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-muted-foreground hover:text-link transition-colors"
                    aria-label="View modrinth.com mod page"
                  >
                    <SiModrinth size={18} />
                  </a>
                  <a
                    href="https://github.com/smit4k/quicknote"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-muted-foreground hover:text-link transition-colors"
                    aria-label="View quicknote on GitHub"
                  >
                    <FaGithub size={18} />
                  </a>
                </div>
                <div className="pr-10">
                  <h3 className="font-medium text-foreground mb-2">Quicknote</h3>
                  <p className="text-sm text-muted-foreground mb-3">
                    Take notes in Minecraft quickly using chat commands
                  </p>
                  <div className="flex gap-2">
                    <span className="text-xs px-2 py-1 bg-[hsl(var(--java))] text-secondary-foreground rounded"><FaJava /></span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div> {/* ✅ closes space-y-8 div */}
        <Footer />
      </div>
    </div>
  );
};

export default Projects;
