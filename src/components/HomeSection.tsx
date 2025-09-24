const HomeSection = () => (
  <div>
    <h2 className="text-xl font-semibold mb-4 text-foreground">Home</h2>
    <p className="text-foreground mb-4">
      Hey 👋, I'm Smit. Welcome to my homepage,{" "}
      <a
        href="https://smit.codes"
        className="text-link hover:text-link-hover transition-colors"
      >
        smit.codes
      </a>
      !
    </p>
    <p className="text-foreground">
      I'm currently a high school student in Michigan who is interested in
      software development, data science and UI/UX design! I also enjoy CAD and
      robotics, and I keep up to date on the latest tech trends and
      innovations.
    </p>
  </div>
);

export default HomeSection;
