import { Link } from "react-router-dom";

const Footer = () => {
  return (
    <div className="mt-8 pt-6 border-t border-border text-sm text-muted-foreground">
      <div className="flex items-center gap-4">
        <span className="space-x-1 flex items-center">
          <span>© 2025 smit4k</span>
          <span>•</span>
          <a
            href="https://github.com/smit4k/smit.codes"
            target="_blank"
            rel="noopener noreferrer"
            className="text-link hover:text-link-hover transition-colors"
          >
            Source
          </a>
        </span>
      </div>
    </div>
  );
};

export default Footer;
