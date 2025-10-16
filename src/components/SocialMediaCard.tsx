// components/SocialMediaButton.tsx

import React from "react";

interface SocialMediaButtonProps {
  name: string;
  handle: string;
  url?: string;
  icon: React.ComponentType<{ size?: number; className?: string }>;
  description: string;
}

const SocialMediaButton: React.FC<SocialMediaButtonProps> = ({
  name,
  handle,
  url,
  icon: Icon,
  description,
}) => {
  const content = (
    <div className="flex items-start gap-3 p-4 border border-border rounded-2xl group hover:bg-secondary transition-colors">
      <Icon size={24} className="text-muted-foreground group-hover:text-link transition-colors mt-1" />
      <div className="flex-1">
        <h3 className="font-medium text-foreground mb-1">{name}</h3>
        <p className="text-sm text-muted-foreground mb-1">
            {handle}
        </p>
        <p className="text-xs text-muted-foreground">{description}</p>
      </div>
    </div>
  );

  return url ? (
    <a href={url} target="_blank" rel="noopener noreferrer" className="active:scale-95 transition-transform">
      {content}
    </a>
  ) : (
    <div>{content}</div>
  );
};

export default SocialMediaButton;
