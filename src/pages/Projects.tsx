import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import ProfilePicture from "@/components/ProfilePicture";
import ProjectSection from "@/components/ProjectSection";
import ProjectCard from "@/components/ProjectCard";

import { FaGithub, FaGlobe, FaRust, FaReact, FaJava } from "react-icons/fa";
import { RiTailwindCssFill } from "react-icons/ri";
import { SiModrinth } from "react-icons/si";
import MainContainer from "@/components/MainContainer";

const Projects = () => {
  return (
    <MainContainer>
        <div className="flex justify-between items-start mb-8">
          <ProfilePicture />
          <Navigation />
        </div>
        <ProjectSection />
        <div className="space-y-4">
          <ProjectCard
            name="conversia"
            description="A powerful, multi-purpose Discord file utility bot using the serenity and poise frameworks"
            links={[
              {
                href: "https://github.com/smit4k/conversia",
                icon: <FaGithub size={18} />,
                label: "View conversia on GitHub",
              },
            ]}
            languages={[
              {
                name: "Rust",
                icon: <FaRust />,
                bgColor: "hsl(var(--rust))",
                textColor: "var(--secondary-foreground)",
              },
            ]}
          />

          <ProjectCard
            name="smit.codes"
            description="This website, a personal homepage, and my first time using React and TailwindCSS"
            links={[
              {
                href: "https://smit.codes",
                icon: <FaGlobe size={18} />,
                label: "View smit.codes site",
              },
              {
                href: "https://github.com/smit4k/smit.codes",
                icon: <FaGithub size={18} />,
                label: "View smit.codes on GitHub",
              },
            ]}
            languages={[
              {
                name: "React",
                icon: <FaReact />,
                bgColor: "hsl(var(--lang-react))",
                textColor: "hsl(var(--lang-react-foreground))",
              },
              {
                name: "TailwindCSS",
                icon: <RiTailwindCssFill />,
                bgColor: "hsl(var(--lang-tailwind))",
                textColor: "hsl(var(--lang-tailwind-foreground))",
              },
            ]}
          />

          <ProjectCard
            name="Quicknote"
            description="Take notes in Minecraft quickly using chat commands"
            links={[
              {
                href: "https://modrinth.com/mod/quicknote",
                icon: <SiModrinth size={18} />,
                label: "View modrinth.com mod page",
              },
              {
                href: "https://github.com/smit4k/quicknote",
                icon: <FaGithub size={18} />,
                label: "View quicknote on GitHub",
              },
            ]}
            languages={[
              {
                name: "Java",
                icon: <FaJava />,
                bgColor: "hsl(var(--java))",
                textColor: "var(--secondary-foreground)",
              },
            ]}
          />
        </div>
        <Footer />
      </MainContainer>
  );
};

export default Projects;
