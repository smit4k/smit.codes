import React from "react";

type Link = {
  href: string;
  icon: React.ReactNode;
  label: string;
};

type Language = {
  name: string;
  icon?: React.ReactNode;
  bgColor?: string;
  textColor?: string;
};

type ProjectCardProps = {
  name: string;
  description: string;
  languages: Language[];
  links: Link[];
  className?: string;
};

const ProjectCard: React.FC<ProjectCardProps> = ({
  name,
  description,
  languages,
  links,
  className = "",
}) => {
  return (
    <div className={`border border-border rounded-2xl p-4 relative ${className}`}>
      <div className="absolute top-4 right-4 flex gap-2">
        {links.map(({ href, icon, label }, i) => (
          <a
            key={i}
            href={href}
            target="_blank"
            rel="noopener noreferrer"
            className="text-muted-foreground hover:text-link transition-colors"
            aria-label={label}
          >
            {icon}
          </a>
        ))}
      </div>
      <div className="pr-16">
        <h3 className="font-medium text-foreground mb-2">{name}</h3>
        <p className="text-sm text-muted-foreground mb-3">{description}</p>
        <div className="flex gap-2 flex-wrap">
          {languages.map(({ name, icon, bgColor, textColor }, i) => (
            <span
              key={i}
              className="text-xs px-2 py-1 rounded-2xl flex items-center gap-1"
              style={{
                backgroundColor: bgColor,
                color: textColor,
              }}
            >
              {icon}
              {name}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
};

export default ProjectCard;
