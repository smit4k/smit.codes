import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import { FaReact } from "react-icons/fa";
import { RiTailwindCssFill } from "react-icons/ri";
import ProfilePicture from "@/components/ProfilePicture";
import HomeSection from "@/components/HomeSection";
import ContactSection from "@/components/ContactSection";

const Index = () => {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4">
      <div className="bg-card border border-border rounded-lg p-8 max-w-2xl w-full">
        <div className="flex justify-between items-start mb-8">
          <ProfilePicture />
          <Navigation />
        </div>
        <div className="space-y-8">
          <HomeSection />
          <ContactSection />
        </div>
        <Footer />
      </div>
    </div>
  );
};

export default Index;