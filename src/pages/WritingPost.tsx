import { useParams, Link } from "react-router-dom";
import Navigation from "@/components/Navigation";
import Footer from "@/components/Footer";
import MakingOfLQF from "../writing/making-of-lqf.mdx";
import { FaClock, FaReact } from "react-icons/fa";
import { RiTailwindCssFill } from "react-icons/ri";

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
      </div>
    </div>
  );
};

export default WritingPost;
