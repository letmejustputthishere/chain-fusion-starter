import { GITHUB_LICENSE, GITHUB_SOURCE_CODE } from "../utils/constants";

export function Info() {
  return (
    <div className="steps-container heading-text info">
      Info: This tool is provided as is under the{" "}
      <a className="link" href={GITHUB_LICENSE}>
        BSD 2-Clause License
      </a>
      . It is still under development and comes with no guarantees. You can find
      the source code, open issues and contribute{" "}
      <a className="link" href={GITHUB_SOURCE_CODE}>
        here.
      </a>{" "}
    </div>
  );
}
