# RustyOps

![Language][language]
![Build Status][build-status]
[![codecov](https://codecov.io/gh/pw-order-of-devs/rusty_ops/graph/badge.svg?token=UX8BOW5HOJ)](https://codecov.io/gh/pw-order-of-devs/rusty_ops)
[![MIT License][license-shield]][license-url]\
[![Contributors][contributors-shield]][contributors-url]
[![Issues][issues-shield]][issues-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]\
[![LinkedIn][linkedin-shield]][linkedin-url]

---

`rusty_ops` - CI/CD system written in Rust

---

## Setup Instructions

[link to doc]\
Include instructions on how to set up your project here. Discuss any environment specifics, dependencies, and how each module is to be configured and ran.

---

## Architecture

[todo] link to architecture docs

The project integrates several modules:

### Services:
* **init**: initialization service [[docs](docs/modules/init.md)]
* **server**: backend server application [[docs](docs/modules/server.md)]
* **agent**: runner service - pipelines execution [[docs](docs/modules/agent.md)]
* **web ui**: [link to doc]

### Libraries:
* **auth**: user authentication module [[docs](docs/modules/auth.md)]
* **commons**: shared functionalities [[docs](docs/modules/commons.md)]
* **domain**: `rusty_ops` domain models [[docs](docs/modules/domain.md)]
* **persist**: database communication layer [[docs](docs/modules/persist.md)]

### Macros:
* **auth_macro**: decorator for securing functions [[docs](docs/modules/auth_macro.md)]

## Contributing

We appreciate all contributions, big or small.

To start contributing, please follow the steps below to set yourself up:

1. **Fork the Repository** - This will create a copy of the project under your own account where you can implement and test your changes.
2. **Create a New Branch** - Branch off the `master` branch on your forked project. It's good practice to name the branch relevant to the feature you're planning to implement. For example, you can use the command `git checkout -b feature/AmazingFeature`.
3. **Implement Your Changes** - On your new branch, go ahead and implement the feature or fix the bug that you are interested in. Always make sure your code is clear and understandable.
4. **Commit Your Changes** - Once you've implemented your changes, you should Commit them with a clear and descriptive commit message like so: `git commit -m 'Add some AmazingFeature'`.
5. **Push Your Changes** - Push your changes to your forked repository by using the command `git push origin feature/AmazingFeature`.
6. **Open a Pull Request** - Finally, log on to GitHub and navigate to the original repository you created your fork from. GitHub will automatically detect the new branch on your fork and give you an option to create a new Pull Request.

Remember to follow these best practices when contributing:

- Always write clear and detailed commit messages.
- Keep changes focused and scoped to a single feature or bug fix.
- Update any associated documentation with your changes.
- Always respect the code of conduct and the community.

Thank you for considering a contribution to this project!

---

## License

This project is licensed under MIT license. See [LICENSE][license-url] for more informations.

---

## Maintainers

* Paweł Walus ([@pw-order-of-devs](http://github.com/pw-order-of-devs))

<!-- links -->
[language]: https://img.shields.io/badge/language-Rust-orange?style=flat-square
[build-status]: https://img.shields.io/github/actions/workflow/status/pw-order-of-devs/rusty_ops/default.yaml?branch=master&style=flat-square
[contributors-shield]: https://img.shields.io/github/contributors/pw-order-of-devs/rusty_ops?style=flat-square
[contributors-url]: https://github.com/pw-order-of-devs/rusty_ops/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/pw-order-of-devs/rusty_ops?style=flat-square
[forks-url]: https://github.com/pw-order-of-devs/rusty_ops/network/members
[stars-shield]: https://img.shields.io/github/stars/pw-order-of-devs/rusty_ops?style=flat-square
[stars-url]: https://github.com/pw-order-of-devs/rusty_ops/stargazers
[issues-shield]: https://img.shields.io/github/issues/pw-order-of-devs/rusty_ops?style=flat-square
[issues-url]: https://github.com/pw-order-of-devs/rusty_ops/issues
[license-shield]: https://img.shields.io/github/license/pw-order-of-devs/rusty_ops?style=flat-square
[license-url]: https://github.com/pw-order-of-devs/rusty_ops/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black?logo=linkedin&colorB=555&style=flat-square
[linkedin-url]: https://www.linkedin.com/in/pawe%C5%82-walus-23121697/
