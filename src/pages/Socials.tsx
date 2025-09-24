import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import { FaGithub, FaDiscord, FaMastodon, FaTwitch, FaReact } from "react-icons/fa";
import { FaXTwitter } from "react-icons/fa6";
import { IoMdMail } from "react-icons/io";
import SocialMediaButton from "@/components/SocialMediaButton";

const Socials = () => {
  const socials = [
    { 
      name: "GitHub", 
      handle: "@smit4k",
      url: "https://github.com/smit4k", 
      icon: FaGithub,
      description: "Open source projects and code"
    },
    { 
      name: "Discord", 
      handle: "sm.it",
      url: "https://discord.com/users/562359659391090689", 
      icon: FaDiscord,
      description: "Direct chat and collaboration"
    },
    { 
      name: "Email", 
      handle: "smit@smit.codes",
      url: "mailto:smit@smit.codes", 
      icon: IoMdMail,
      description: "Email me!"
    },
    { 
      name: "X", 
      handle: "@saberdevx",
      url: "https://twitter.com/saberdevx", 
      icon: FaXTwitter,
      description: "Tech thoughts and updates"
    },
  ];

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
            <h2 className="text-xl font-semibold mb-4 text-foreground">Socials</h2>
            <p className="text-muted-foreground mb-6">
              Connect with me across different platforms. Feel free to reach out for collaborations, 
              questions, or just to say hi!
            </p>
            <div className="grid gap-4 md:grid-cols-2">
              {socials.map((social, index) => (
                <SocialMediaButton
                  key={index}
                  name={social.name}
                  handle={social.handle}
                  url={social.url}
                  icon={social.icon}
                  description={social.description}
                />
              ))}
            </div>
          </div>
        </div>
        <Footer />
      </div>
    </div>
  );
};

export default Socials;
