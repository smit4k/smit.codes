import React from "react";

type Props = {
  children: React.ReactNode;
  className?: string;
};

/**
 * PageTransition
 * Simple mount animation wrapper that applies a CSS class for enter animation.
 */
const PageTransition = ({ children, className = "" }: Props) => {
  return (
    <div className={["page-enter", className].join(" ")}>
      {children}
    </div>
  );
};

export default PageTransition;
