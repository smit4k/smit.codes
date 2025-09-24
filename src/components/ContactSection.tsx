const ContactSection = () => (
        <div>
            <h2 className="text-xl font-semibold mb-4 text-foreground">Contact</h2>
            <p className="text-foreground mb-4">
              If you have any questions, comments, or just want to talk, you can contact me easily through
              the following ways:
            </p>
            <div className="space-y-2 text-foreground">
              <p className="flex items-center gap-2">
                <span>Discord:</span>
                <span className="bg-teal-500/10 border border-teal-500/60 text-teal-300 px-2 py-0.5 rounded text-sm font-mono">
                  sm.it
                </span>
              </p>
              <p>
                Email:{" "}
                <a
                  href="mailto:smit@smit.codes"
                  className="text-link hover:text-link-hover transition-colors"
                >
                  smit@smit.codes
                </a>
              </p>
            </div>
        </div>
);

export default ContactSection;