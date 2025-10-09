import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import ProfilePicture from "@/components/ProfilePicture";
import HomeSection from "@/components/HomeSection";
import ContactSection from "@/components/ContactSection";
import MainContainer from "@/components/MainContainer";

const Index = () => {
  return (
    <MainContainer>
      <div className="flex justify-between items-center mb-8">
        <ProfilePicture />
          <Navigation />
        </div>
        <div className="space-y-8">
          <HomeSection />
          <ContactSection />
        </div>
        <Footer />
    </MainContainer>
  );
};

export default Index;