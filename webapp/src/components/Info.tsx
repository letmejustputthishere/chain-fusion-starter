import { GITHUB_LICENSE, GITHUB_SOURCE_CODE } from "../utils/constants";

export function Info() {
  return (
    <div className="steps-container heading-text info">
      DISCLAIMER: This tool is provided as is under the{" "}
      <a className="link" href={GITHUB_LICENSE}>
        BSD 2-Clause License
      </a>
      . It is in it's infancy and comes with no guarantees. Consider it alpha
      and do not use it for production. You can find the source code, open
      issues and contribute{" "}
      <a className="link" href={GITHUB_SOURCE_CODE}>
        here.
      </a>{" "}
      <br />
      The{" "}
      <a
        className="link"
        href="https://commons.wikimedia.org/wiki/File:Eo_circle_pink_white_repeat-one.svg"
      >
        favicon
      </a>{" "}
      was created by Emoji One.
    </div>
  );
}
