import { useParams, Link } from "react-router-dom";
import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import MakingOfLQF from "../writing/making-of-lqf.mdx";
import { FaClock, FaReact } from "react-icons/fa";
import ProfilePicture from "@/components/ProfilePicture";
import MainContainer from "@/components/MainContainer";
import WritingView from "@/components/WritingContainer";
import WritingContainer from "@/components/WritingContainer";

const posts = {
  "making-of-lqf": {
    component: MakingOfLQF,
    title: "My Own Configuration Language, LQF",
    date: "July 9, 2025",
    estTimeToRead: "3 min",
  },
};

const WritingPost = () => {
  const { slug } = useParams();
  const post = posts[slug as keyof typeof posts];

  if (!post) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-foreground">Post not found.</div>
      </div>
    );
  }

  const PostComponent = post.component;

  return (
    <WritingContainer>
      <div className="flex justify-between items-center mb-8"> 
        <ProfilePicture />
        <Navigation />
      </div>
      <Link to="/writing" className="text-link hover:underline mb-4 block">
          &larr; Back to Writing
        </Link>
        <h1 className="text-2xl font-bold mb-2 text-foreground">{post.title}</h1>
        <div className="flex justify-between items-center text-xs text-muted-foreground mb-2">
          <span>{post.date}</span>
          <span className="flex items-center space-x-1">
            <span>{post.estTimeToRead}</span>
            <FaClock className="h-4 w-4 text-muted-foreground" aria-hidden="true" />
          </span>
        </div>
        <hr className="border-border mb-4" />
        <div className="prose prose-invert">
          <PostComponent />
        </div>
        <Footer />
      </WritingContainer>
  );
};

export default WritingPost;
