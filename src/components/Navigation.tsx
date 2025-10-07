import { Link, useLocation } from "react-router-dom";

const Navigation = () => {
  const location = useLocation();

  const isActive = (path: string) => location.pathname === path;

  return (
    <nav className="flex flex-wrap justify-end gap-2 w-full md:w-auto">
      {[
        { path: "/", label: "home" },
        { path: "/projects", label: "projects" },
        { path: "/writing", label: "writing" },
        { path: "/socials", label: "socials" },
      ].map(({ path, label }) => (
        <Link
          key={path}
          to={path}
          aria-current={isActive(path) ? "page" : undefined}
          className={`px-3 py-1 rounded-2xl transition-colors text-sm ${
            isActive(path)
              ? "text-accent bg-secondary font-semibold"
              : "text-muted-foreground hover:text-link"
          }`}
        >
          {label}
        </Link>
      ))}
    </nav>
  );
};

export default Navigation;
