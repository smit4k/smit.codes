import Navigation from "@/components/Navigation";
import { FaClock, FaReact } from "react-icons/fa";
import Footer from "@/components/Footer";
import ProfilePicture from "@/components/ProfilePicture";
import WritingSection from "@/components/WritingSection";
import MainContainer from "@/components/MainContainer";

const posts = [
  // Add more posts here
  {
    slug: "making-of-lqf",
    title: "My Own Configuration Language, LQF",
    date: "July 9, 2025",
    description: "How I designed my own configuration language and my thought process behind it.",
    estTimeToRead: "3 min",
  },
];

const Writing = () => (
  <MainContainer>
    <div className="flex justify-between items-center mb-8">
      <ProfilePicture />
      <Navigation />
    </div>
    <WritingSection />
    <div className="space-y-4">
      {posts.map((post) => (
        <a
          key={post.slug}
          href={`/writing/${post.slug}`}
          className="block border border-border rounded-2xl p-4 hover:bg-secondary transition-colors"
        >
          <div className="flex justify-between items-start">
            <div>
              <h3 className="text-lg font-semibold text-foreground mb-1">
                {post.title}
              </h3>
              <p className="text-xs text-muted-foreground mb-2">{post.date}</p>
            </div>
            <span className="text-xs text-muted-foreground whitespace-nowrap ml-4 mt-1">
              {post.estTimeToRead} <FaClock className="inline-block" />
            </span>
          </div>
          <p className="text-sm text-muted-foreground mt-2">{post.description}</p>
        </a>
      ))}
    </div>
    <Footer />
  </MainContainer>
);

export default Writing;
