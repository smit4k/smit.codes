import React from "react";

const ProfilePicture: React.FC = () => {
  return (
    <img
      src="/smit_pfp.svg"
      alt="Smit's profile picture"
      className="w-10 h-10 rounded-full object-cover border-2 border-border profile-spin hover:cursor-pointer"
    />
  );
};

export default ProfilePicture;
