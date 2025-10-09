import { useLocation } from "react-router-dom";
import { useEffect } from "react";
import { FaHome } from "react-icons/fa";

const NotFound = () => {
  const location = useLocation();

  useEffect(() => {
    console.error(
      "404 Error: User attempted to access non-existent route:",
      location.pathname
    );
  }, [location.pathname]);

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-background text-foreground p-4">
      <div className="text-center mb-8">
        <h1 className="text-4xl font-bold mb-4">404</h1>
        <p className="text-xl text-muted-foreground mb-6">Oops! Page not found</p>
        <a
  href="/"
  className="inline-flex items-center justify-center p-2 rounded-md bg-card border border-border text-foreground transition-colors hover:bg-muted hover:text-foreground"
  aria-label="Return to Home"
>
  <FaHome className="w-5 h-5" />
</a>

      </div>

      {/* GitHub Contribution Snake */}
      <div className="w-full max-w-2xl">
        <img
          src="https://raw.githubusercontent.com/smit4k/smit.codes/refs/heads/output/github-contribution-grid-snake-dark.svg"
          alt="GitHub Contribution Snake"
          className="w-full h-auto"
          style={{ filter: 'drop-shadow(0 4px 6px rgba(0, 0, 0, 0.4))' }}
        />
      </div>

      <p className="mt-6 text-sm text-muted-foreground">
        Check out my GitHub contributions while you're here!
      </p>
    </div>
  );
};

export default NotFound;
