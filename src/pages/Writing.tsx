import Navigation from "@/components/Navigation";
import { FaClock, FaReact } from "react-icons/fa";
import { RiTailwindCssFill } from "react-icons/ri";
import Footer from "@/components/Footer";
import ProfilePicture from "@/components/ProfilePicture";

const posts = [
  {
    slug: "making-of-lqf",
    title: "My Own Configuration Language, LQF",
    date: "July 9, 2025",
    description: "How I designed my own configuration language and my thought process behind it.",
    estTimeToRead: "3 min",
  },
  // Add more posts here
];

const Writing = () => (
  <div className="min-h-screen bg-background flex items-center justify-center p-4">
    <div className="bg-card border border-border rounded-lg p-8 max-w-2xl w-full">
      <div className="flex justify-between items-start mb-8">
        <ProfilePicture />
        <Navigation />
      </div>
      <h2 className="text-xl font-semibold mb-4 text-foreground">Writing</h2>
      <p className="text-muted-foreground mb-6">
        Welcome to my writing! I'll be sharing some cool stuff!
      </p>
      <div className="space-y-4">
        {posts.map((post) => (
          <a
            key={post.slug}
            href={`/writing/${post.slug}`}
            className="block border border-border rounded-lg p-4 hover:bg-secondary transition-colors"
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
    </div>
  </div>
);

export default Writing;
