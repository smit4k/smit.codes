import React from "react";
import PageTransition from "@/components/PageTransition";
import ParticlesBackground from "@/components/Particles";

type Props = {
  children: React.ReactNode;
  className?: string;
};

/**
 * MainContainer
 * Reusable page container that provides the common outer and inner card layout
 * used across pages. Accepts additional classes for the inner card via
 * `className` and renders children inside.
 */
const MainContainer = ({ children, className = "" }: Props) => {
  return (
    <div className="min-h-screen bg-background flex items-center justify-center p-4 relative">
      <ParticlesBackground />
      <div className={["bg-card/80 backdrop-blur-sm border border-border rounded-2xl p-8 max-w-4xl w-full relative z-10", className].join(" ")}>
        <PageTransition>
          {children}
        </PageTransition>
      </div>
    </div>
  );
};

export default MainContainer;
