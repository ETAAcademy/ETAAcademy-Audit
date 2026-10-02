# ETAAcademy-Audit: 49. Linux in Practice

<table>
  <tr>
    <th>title</th>
    <th>tags</th>
  </tr>
  <tr>
    <td>49 LNX</td>
    <td>
      <table>
        <tr>
          <th>audit</th>
          <th>basic</th>
          <th>article</th>
          <td>LNX</td>
        </tr>
      </table>
    </td>
  </tr>
</table>

[Github](https://github.com/ETAAcademy)｜[Twitter](https://twitter.com/ETAAcademy)｜[ETA-Audit](https://github.com/ETAAcademy/ETAAcademy-Audit)

Authors: [Evta](https://twitter.com/pwhattie), looking forward to your joining

# Linux in Practice: From Boot and Desktop to Shell, Networks, and Text

Linux becomes easier to use when its parts fit into a single picture. The kernel manages hardware and processes; a distribution supplies the surrounding software; filesystems expose storage through one directory tree; and shells, desktops, and services provide ways to work with that system. Administration then becomes a matter of understanding those relationships and choosing the right tool.

Effective Linux work starts with identifying which part of the system answers the question. Use mount information for storage, process state for running work, routes and resolver settings for connectivity, and ownership and permissions for access. A desktop, shell, or remote session gives a different interface to those same underlying resources.

For changes, inspect the input, preview the intended operation, and check the result. Review patches before applying them, preview synchronization against the actual destination, and inspect intermediate pipeline output before trusting a report. Keep recoverable versions of important data. These habits make the commands useful as a connected workflow.

## 1. Distributions, Storage, and the Boot Process

### What a distribution provides

Strictly speaking, **Linux is the kernel**. A **distribution** combines it with system libraries, utilities, package management, services, and applications. Desktop installations also include a graphics system and desktop environment. Servers and embedded systems can omit those graphical components, while minimal installations may leave out development tools and local documentation.

The kernel schedules tasks, manages memory, and mediates access to devices. Libraries such as glibc provide interfaces and runtime support for programs. Utilities, compilers, and debuggers handle everyday administration and development. Package managers install and update these components using repositories maintained for the distribution. The result depends as much on the maintainers' version choices, defaults, and support policy as on the kernel itself.

Related distributions often share package formats and administration conventions:

| Family  | Examples                                  |
| :------ | :---------------------------------------- |
| Red Hat | Fedora, CentOS Stream, RHEL, Oracle Linux |
| SUSE    | SLES, openSUSE Leap, Tumbleweed           |
| Debian  | Debian, Ubuntu, Ubuntu-based Linux Mint   |

A package format describes the bundle and its metadata. Higher-level managers also resolve dependencies and work with repositories. Sharing RPM or DEB does not mean two distributions have interchangeable packages: repositories, library versions, architecture, and support policies still matter.

**Upstream** and **downstream** describe how work flows between projects. Fedora integrates changes that can feed later enterprise releases; CentOS Stream sits immediately ahead of upcoming RHEL releases. Historical CentOS Linux instead rebuilt released RHEL sources downstream. [Red Hat's CentOS explanation](https://www.redhat.com/en/topics/linux/what-is-centos) distinguishes those roles, while [Fedora's account of CentOS Stream](https://fedoramagazine.org/fedora-and-centos-stream/) explains the broader development relationship.

Ubuntu builds on Debian but follows its own integration and release process. During development, it imports many packages from Debian unstable, so describing it simply as a derivative of Debian stable obscures how it is made. See [Ubuntu's Debian relationship](https://ubuntu.com/project/docs/community/governance/debian/) and its package import process. In the SUSE family, Tumbleweed follows a rolling release model, while Leap's relationship with the enterprise base varies by generation; the [openSUSE development documentation](https://osrt.opensuse.org/docs/processes.html) describes these processes.

**Binary compatibility** means a compiled program runs in another supported environment without rebuilding. It depends on processor architecture, libraries, and the application binary interface, or ABI. Identical sources can produce incompatible binaries, while different versions can preserve a compatible ABI. Use the intended release's compatibility commitments, when evaluating software support.

### How programs and users share the system

Linux supports preemptive multitasking: the kernel can interrupt one task to run another. Multiple users share resources through identities and permissions. These controls define access boundaries, while processes, services, and graphical applications all run within the operating system.

A **daemon** provides a background service, such as a web server or time synchronization. It may start during boot or on demand. Names ending in `d`, such as `httpd`, follow a convention rather than a technical requirement.

A terminal supplies a place to enter and display text; a shell such as Bash or Zsh interprets commands. The shell parses input, expands variables and patterns, arranges redirection, and runs a builtin or launches an external program. Programs use system calls to request kernel services. Their output can go to the terminal, a file, or another process.

A graphical application uses a display system such as X11 or Wayland. A desktop environment adds panels, settings, and a coordinated workspace. Desktop programs, shells, and services cooperate, but graphical applications do not all pass through a shell. A headless server can provide services and command-line access without a desktop.

### Storage appears as one directory tree

Linux paths belong to a tree rooted at `/`. A **mount point** attaches another filesystem to that tree, so a disk partition, network share, or memory-backed filesystem can appear under an ordinary directory. The kernel's Virtual File System provides a common interface across filesystem implementations.

A partition defines a range of disk sectors; a filesystem organizes files and directories. A filesystem often lives inside a partition, but a partition can instead hold swap or participate in Logical Volume Management. Filesystems can also live on logical volumes, and virtual filesystems need no disk partition. Symbolic links refer to paths and can cross mount boundaries.

| Location                      | Main purpose                                         |
| :---------------------------- | :--------------------------------------------------- |
| `/boot`                       | Kernel images and boot files                         |
| `/etc`                        | System-wide configuration                            |
| `/home`                       | Ordinary users' home directories                     |
| `/var`                        | Changing data, including logs, queues, and databases |
| `/usr`                        | Installed applications, libraries, and shared data   |
| `/dev`                        | Device nodes                                         |
| `/proc`                       | Process and kernel information                       |
| `/sys`                        | Devices and other kernel objects                     |
| `/run`                        | Runtime data generally recreated at boot             |
| `/media`, `/run/media/<user>` | Common removable-media mount locations               |

The phrase “everything is a file” captures Linux's many file-like interfaces. It does not mean that a device node, socket, or `/proc` entry is an ordinary document stored on disk. Likewise, `/usr` contains installed software; personal documents normally belong in a user's home directory.

An absolute path starts with `/`; a relative path starts from the working directory. Mounting a USB drive at `/run/media/student/Fedora` makes its `readme.txt` accessible as `/run/media/student/Fedora/readme.txt`. Names are typically case-sensitive, although behavior depends on the filesystem and configuration.

Common filesystems include ext4 and XFS for general storage, Btrfs for copy-on-write features such as snapshots, FAT variants for compatibility, and tmpfs for temporary memory-backed storage that may use swap. Distribution defaults vary by release. Inspect the actual system:

```bash
$ df -T -h
```

GNU `df` uses `-T` to display filesystem types and `-h` for human-readable sizes. Its mount locations connect paths to their underlying storage; entries do not necessarily correspond to individual disk partitions.

### From firmware to the login prompt

A conventional GRUB-based boot follows this sequence: firmware initializes the machine, a bootloader loads the kernel, early userspace prepares the root filesystem, and the main init system starts services and login facilities. The sequence is a troubleshooting model; actual mechanisms vary and some stages overlap.

Traditional BIOS boot reads initial code from the Master Boot Record. UEFI can load an EFI executable from an EFI System Partition, commonly formatted as FAT32. The conventional MBR occupies 512 bytes, including four partition entries and a boot signature; its partition-count and familiar capacity limits belong to the partition format, not to every BIOS system. GPT provides another partition-table format and is commonly paired with UEFI. The [UEFI specification's legacy MBR section](https://uefi.org/specs/UEFI/2.11/05_GUID_Partition_Table_Format.html) gives the layout.

GRUB can offer an operating-system menu and load both a kernel and an initial RAM filesystem. After receiving control, the kernel initializes memory management, scheduling, and drivers. A typical installation then uses an **initramfs**, an archive unpacked into memory, to supply the tools needed before the real root filesystem becomes accessible.

Those tools may load storage drivers, assemble logical volumes, unlock encrypted storage, or establish networking. The kernel can run `/init` from this early filesystem as its first userspace process; that process prepares the real root and transfers control to its init program. Initramfs differs from the older initrd, which used a RAM-backed block device. The [kernel's initramfs documentation](https://docs.kernel.org/filesystems/ramfs-rootfs-initramfs.html) explains the handoff.

On systems using **systemd** as their main init system, it runs as PID 1 and manages services and other units. Targets such as `multi-user.target` and `graphical.target` group units for an operating state. Independent services can start concurrently, while ordering constraints coordinate dependent work.

Requirement and ordering dependencies are separate: needing another unit does not automatically specify which starts first. Real startup time depends on dependency chains, resource contention, firmware, and storage. A simple comparison of serial and parallel service durations cannot predict total boot time.

Common service operations are:

```bash
$ systemctl start <service>
$ systemctl stop <service>
$ systemctl enable <service>
$ systemctl status <service>
```

`start` and `stop` change the current running state. `enable` creates installation links, commonly used to pull the unit in at boot; it does not immediately start the service. `status` reports its current state and may include recent logs. Changes to system services generally require administrative authorization.

### Installation choices that affect later work

Choose a distribution by workload, required applications, hardware support, update frequency, maintenance period, and available memory and disk space. Architecture support and lifecycle commitments belong to specific releases, so check the version being installed.

A virtual machine provides a virtual disk while retaining the host system. A live USB runs an environment without first installing it internally. Either can still write through shared folders, attached disks, persistence, or an installer. Dual booting installs another operating system alongside the existing one and can alter partitions and boot configuration; back up data before those changes.

A simple installation puts most files under a root filesystem, configures swap as appropriate, and uses an EFI System Partition when required. Separate `/home` storage can simplify reinstalls, while a separate `/var` can contain growth from logs and databases. Neither arrangement replaces backups, and a full `/var` can still disrupt services. LVM adds allocation flexibility, but resizing remains constrained by available space and filesystem capabilities.

Repeated installations can use unattended configuration: Kickstart for Anaconda-based installers, AutoYaST for relevant SUSE installers, Preseed for Debian Installer, and Autoinstall YAML for supported modern Ubuntu installers. The mechanism depends on the installer and release.

Create a normal account and configure administration through the distribution's root or `sudo` model. SELinux and AppArmor can add policy restrictions beyond ordinary file permissions. Together, installation choices establish the storage, identity, and service boundaries used throughout daily work.

## 2. Desktop Sessions, Networking, and Software

A Linux desktop connects a graphical workspace to system services, network configuration, software repositories, and applications. Understanding those connections helps explain what happens when you log in, connect to Wi-Fi, install software, or troubleshoot a display. Interfaces vary across distributions, but the underlying responsibilities remain recognizable.

### How a graphical session works

A **display manager**, such as GDM, SDDM, or LightDM, presents the login screen, authenticates users through the system's authentication infrastructure, and launches the selected session. A **window manager** controls window placement and focus; a **session manager** coordinates session components. A **desktop environment**, such as GNOME, KDE Plasma, or Xfce, integrates these components with an interface and applications.

The display server or compositor coordinates application surfaces, input, and screen output. These roles can overlap: GNOME Shell depends closely on Mutter, and a desktop environment already includes several session components. Minimal graphical setups can operate without a full desktop environment or graphical login manager. On systems configured for graphical startup, the display manager starts automatically. Its login screen and the user's desktop may run through separate display-server instances.

**X11** uses a client/server model: the X server manages display and input devices, while applications act as clients. The server usually runs on the machine in front of the user; clients can run locally or remotely. **Wayland** defines communication between applications and a compositor that also acts as the display server. Applications normally render buffers that the compositor combines for output. Both systems rely on graphics libraries, drivers, and Linux graphics infrastructure; performance depends on the complete implementation. See [Wayland's architecture documentation](https://wayland.freedesktop.org/architecture.html).

Ordinary Wayland clients do not receive unrestricted global access to input or other applications' surfaces through the core protocol. Screen sharing therefore requires supported interfaces and policy decisions. X11 applications can run through Xwayland in a Wayland session, so session type alone does not determine application compatibility. The Wayland FAQ explains these distinctions.

From a suitable text console, an installed and configured X11 environment can be started manually:

```bash
$ startx
```

`startx` uses `xinit` to start an X server and initial client or session; `~/.xinitrc` can control what starts. It is not a general Wayland startup command.

### Managing sessions and files

In GNOME, pressing **Super** opens the Activities overview, where typing searches for applications. Launchers commonly use `.desktop` files to associate program names, icons, and launch information. Favorites provide quick access, while system controls expose connectivity, settings, and power actions.

Session actions preserve different parts of your work:

| Action      | Effect                                                               |
| :---------- | :------------------------------------------------------------------- |
| Lock        | Keep the session running and require authentication to return        |
| Switch user | Leave the existing session available while opening another           |
| Log out     | End the desktop session and normally close its applications          |
| Suspend     | Enter a low-power state while retaining information needed to resume |
| Hibernate   | Save state to persistent storage and power down, when supported      |

GNOME documents **Super+L** for locking; customized bindings can differ. Logging out may leave background services running, depending on cleanup policies. Suspend can use suspend-to-idle rather than ACPI S3. Multiple sessions share the kernel, hardware, and system services, and sessions belonging to one user share that identity's access rights. They do not provide the isolation of separate virtual machines. See [GNOME's keyboard shortcuts](https://help.gnome.org/gnome-help/shell-keyboard-shortcuts.html).

GNOME Files, also called Nautilus, shows home directories, mounted storage, and other locations. Home directories usually follow `/home/<username>`; names beginning with a dot are hidden by convention. Directories such as `.config` and `.local` hold settings and user data. **Ctrl+H** commonly reveals hidden files.

Moving files to Trash usually allows recovery. **Shift+Delete** bypasses Trash, and emptying Trash removes its contents. Supported actions depend on the storage location and installed version; consult the application's menus and GNOME's deletion guide.

### Displays and clock settings

Settings groups common controls for displays, input devices, users, networking, and time. GNOME Tweaks exposes additional options, while GNOME Extensions manages extensions that change the Shell interface. Their availability and features depend on the release.

Display settings control several independent properties: resolution selects pixel dimensions, refresh rate specifies refreshes per second, scaling changes the apparent size of interface elements, and orientation rotates the display. Multiple monitors can extend or mirror a workspace; Night Light changes color temperature. A 60 Hz display need not receive 60 application frames per second. Settings commonly revert an unconfirmed display change, allowing recovery from an unreadable mode.

X.Org supports these configuration paths:

```
/etc/X11/xorg.conf           ← legacy single-file config
/etc/X11/xorg.conf.d/*.conf  ← modern drop-in fragment directory
```

Many installations use automatic detection. Explicit files remain useful for particular hardware requirements, while native Wayland compositors use their own configuration mechanisms.

Clock configuration involves the hardware clock, the running system clock, and the timezone used to display timestamps. Linux normally keeps system wall-clock time on a UTC-based timeline; the hardware clock can represent UTC or local time. For a particular instant:

$$
T_{\text{local}} = T_{\text{UTC}} + \Delta_{\text{TZ}}
$$

Here, $\Delta_{\text{TZ}}$ is the applicable timezone offset. Named timezones include rules such as daylight saving changes, so they carry more information than a fixed offset. System timezone configuration commonly uses `/etc/localtime` and `/usr/share/zoneinfo`. Applications and logs may use different timestamp formats or offsets; check these before comparing events.

Services such as `chronyd` and `systemd-timesyncd` synchronize clocks with network sources. Reference clocks are conventionally called stratum 0, directly connected servers stratum 1, and servers synchronized from those stratum 2. NTP **stratum** describes synchronization distance from a reference clock, rather than guaranteeing accuracy. Network delay, asymmetry, oscillator quality, and source selection affect the error a client observes. A lower stratum does not automatically provide better time. See the [NTP FAQ](https://www.ntp.org/ntpfaq/ntp-s-algo/) and server-stratum guide.

### Network configuration and identity

Many desktops use **NetworkManager** to coordinate devices and saved connection profiles. Ethernet often obtains configuration automatically; Wi-Fi adds network selection and authentication. Mobile broadband requires provider settings, and VPN connections may require plugins. Whether a change needs administrative authentication depends on system policy and whether the profile belongs to one user or the whole system.

A typical IPv4 profile contains an address, subnet mask or prefix, gateway, and DNS settings. The address identifies the interface, the prefix defines its local subnet, the gateway routes traffic elsewhere, and DNS resolves names. DHCP commonly supplies these settings automatically; static profiles store manually chosen values.

For example, `192.168.1.100/24` uses the mask `255.255.255.0`, with `192.168.1.1` as a possible gateway. Actual values must match the network. Local communication may work without a default gateway, and communication by address does not require DNS. IPv6 also offers automatic configuration beyond DHCP. These distinctions help separate link, addressing, routing, and name-resolution failures.

Interface names often describe hardware location:

```
eno1                ← onboard NIC (index 1)
enp3s0              ← PCI bus 3, slot 0
wlp2s0              ← Wi-Fi, PCI bus 2, slot 0
```

The `en` prefix denotes Ethernet and `wl` denotes wireless LAN. Topology-based names reduce dependence on discovery order, although hardware and naming-policy changes can still alter them. Legacy names such as `eth0` can also be stable when explicitly assigned.

A **MAC address** identifies an interface at the link layer. Ethernet and Wi-Fi commonly use 48-bit addresses, written as six hexadecimal byte pairs. Locally administered and randomized addresses mean a MAC address is neither necessarily a manufacturer identifier nor an unchangeable device identity. NetworkManager's wireless settings document permanent, randomized, and stable generated addresses.

A VPN adds a tunnel and routing policy, commonly with encryption. Supported options include OpenVPN, IPsec/IKEv2, WireGuard, and OpenConnect-compatible connections. Profiles may route only private destinations or most outbound traffic; connecting a VPN alone does not establish which route every application uses. Older material may mention PPTP, a legacy protocol with known weaknesses.

### Packages, repositories, and dependencies

A package combines files with metadata about versions, dependencies, and installation requirements; it may include installation or removal scripts. Repositories publish packages and searchable indexes. Higher-level managers select versions, retrieve files, and resolve dependencies, while installation machinery updates files and package records. Implementations may use libraries directly rather than launching a lower-level command. Lower-level tools can check dependencies even when they cannot automatically retrieve missing packages.

| Distribution family | Format | Lower-level tool | Repository manager |
| :------------------ | :----- | :--------------- | :----------------- |
| Debian and Ubuntu   | `.deb` | `dpkg`           | APT                |
| Fedora and RHEL     | `.rpm` | `rpm`            | DNF                |
| openSUSE and SLES   | `.rpm` | RPM tooling      | Zypper             |

Installing an application also requires its supporting libraries. An email client, for example, may require a TLS library that itself requires a runtime library. A dependency graph captures these relationships:

$$
G = (V, E) \text{ where } V = \{\text{packages}\}, \; E = \{(A \to B) \mid A \text{ depends on } B\}
$$

Packages form the vertices $V$, and dependency relationships form the edges $E$. For fixed dependencies, the required set can be described as:

$$
\text{InstallSet}(P) = \{P\} \cup \bigcup_{Q \in \text{deps}(P)} \text{InstallSet}(Q)
$$

Here, $P$ is the requested package and $\text{deps}(P)$ contains its direct dependencies. The expression describes dependency closure, including dependencies of dependencies. Already installed packages that satisfy requirements need not be downloaded again.

Real resolution must also handle cycles, alternative providers, conflicts, architecture restrictions, and version constraints. A larger version number alone does not guarantee compatibility, and sharing a package format does not make repositories interchangeable. Use packages intended for the distribution and release, or explicitly supported by their publisher. The Debian Policy Manual explains package relationships.

### Common package operations

Package-changing commands require administrative privileges, commonly obtained with `sudo`; queries usually do not. Replace example package names and filenames with real values, and review the proposed transaction before accepting it.

On Debian and Ubuntu, `dpkg` works with local packages and installed records:

```bash
$ dpkg -i package.deb          # Install a local .deb package
$ dpkg -L package-name         # List files installed by a package
```

Missing dependencies can leave a package unpacked but unconfigured. APT adds repository access and dependency resolution:

```bash
$ apt install package-name     # Download + install + resolve deps
$ apt remove package-name      # Remove package (keep config files)
$ apt update                   # Refresh repository package index
$ apt upgrade
$ apt search keyword           # Search repository for packages
```

`update` refreshes available-version indexes; `upgrade` applies eligible upgrades. Packages can be held back when completing an upgrade requires removals or other disallowed changes. Removal generally preserves package-managed configuration; purging provides different cleanup behavior.

Fedora and RHEL use RPM for lower-level operations and DNF for repository transactions:

```bash
$ rpm -ql package-name         # List files in an installed package
$ dnf install package-name     # Download + install + resolve deps
$ dnf remove package-name      # Remove package and unneeded deps
$ dnf update
$ dnf search keyword           # Search repositories
```

Dependency cleanup and upgrades depend on the installed DNF version, configuration, and repository constraints. Older instructions may use `yum`; follow the interface documented for the release.

SUSE systems expose comparable operations through Zypper:

```bash
$ zypper install package-name  # Install
$ zypper remove package-name   # Remove
$ zypper refresh               # Refresh repository index
$ zypper update                # Upgrade installed packages
$ zypper search keyword        # Search
```

`refresh` updates metadata, while `update` performs eligible updates. Rolling releases and distribution upgrades can require different procedures. Where available, YaST provides graphical software management using shared package infrastructure. Tool availability varies: openSUSE Leap 16.0's release notes document its move from YaST to Cockpit for manual administration.

### Graphical software tools and application formats

GNOME Software presents applications, installation controls, and updates. Plugins connect it to sources such as native packages through PackageKit and Flatpak. Ubuntu's separate App Center provides Snap and DEB applications. Synaptic offers a package-oriented APT interface with dependency, version, source, and removal details.

Flatpak and Snap manage application runtimes or dependencies to reduce reliance on host library versions. Compatibility still depends on the application and system, and their update paths differ from native packages. When several formats offer the same application, check its source, format, and permissions to understand how it will be maintained.

### Applications for everyday work

Choose software by the files and workflows it must support. Similar task coverage does not guarantee matching document rendering, plugins, collaboration features, or automation.

| Task                    | Representative applications                                     |
| :---------------------- | :-------------------------------------------------------------- |
| Browsing                | Firefox, Chromium, GNOME Web; Lynx or w3m for terminal browsing |
| Email and file transfer | Thunderbird, Evolution, Mutt; FileZilla                         |
| Office documents        | LibreOffice Writer, Calc, and Impress                           |
| Development             | Vim, Emacs, Visual Studio Code, Eclipse; GCC, Clang, and GDB    |
| Audio and video         | Audacity, VLC, Kdenlive, Blender, FFmpeg                        |
| Graphics and publishing | GIMP, Inkscape, Scribus, ImageMagick                            |

Terminal browsers have limited support for modern web applications. Email access depends on account protocols and provider authentication. IMAP synchronizes server-side mailboxes and message state, making it convenient across devices; POP3 downloads messages without equivalent shared-folder synchronization. IMAP can keep offline copies, and POP3 can leave messages on the server. Sending mail is separate, commonly using SMTP.

LibreOffice handles formats including `.docx`, `.xlsx`, and `.pptx`, but complex layouts, fonts, formulas, and macros may change during conversion. Check representative files in the recipient's application. Development tools likewise serve distinct roles: compilers produce object files, linkers build executables, and debuggers inspect program state. Tools such as `perf`, Valgrind, and `strace` address different performance, memory, and system-interaction questions.

Command-line multimedia tools support repeatable workflows:

```bash
$ ffmpeg -i input.mp4 -c:v libx264 -c:a aac output.mp4
```

This re-encodes video with H.264 and audio with AAC into an MP4 container. `-i` selects the input, `-c:v libx264` selects the video encoder, and `-c:a aac` selects the audio encoder. The output filename selects the container. The FFmpeg build must support the specified encoders and input decoding; settings determine quality, size, and processing time.

For graphics, GIMP edits raster pixels, Inkscape edits vector paths, and Scribus combines text and graphics into page layouts. Enlarging a raster image cannot recover missing detail. Vector paths can be rendered at different sizes, though embedded bitmaps, fonts, effects, and rasterization still influence output. Application availability and maintenance also change.

## 3. Commands, Files, and Pipelines

The command line connects a few recurring ideas: a shell interprets commands, paths locate files, streams carry data between programs, and each running program has an identity and state. Understanding these relationships makes individual commands easier to use and their results easier to interpret.

### Read commands through the shell

A simple command follows this pattern:

```
$ command  [options]  [arguments]
  ^name    ^-p/--print  ^filename
```

The command identifies the operation, options adjust behavior, and arguments supply filenames or other values. Shell input can also contain assignments, redirections, pipelines, functions, and control structures. Short and long options are common conventions, but individual programs can use different rules.

For an external executable named without a slash, the shell normally searches `$PATH`. It can also resolve aliases, functions, and built-ins. Use `type command-name` to see which interpretation is active. Bash's `PS1` controls the main prompt and `PS2` controls continuation prompts. A prompt ending in `#` traditionally indicates root, but its appearance does not establish the current user's privileges.

Linux can also provide virtual terminals: separate console sessions sharing a display and keyboard. A combination such as `Ctrl+Alt+F3` often switches to one, although console assignments vary. Stopping a display-manager service can terminate graphical sessions, so session-management commands require awareness of the work those sessions contain.

### Navigate paths and inspect file names

Every shell has a current working directory. An absolute path starts at `/`; a relative path starts from the current directory. Thus `notes.txt`, `./notes.txt`, and `../notes.txt` describe different relationships to that starting location. An unquoted `~` expands to a home-directory path before the program receives it.

| Command                           | Purpose                                      |
| :-------------------------------- | :------------------------------------------- |
| `pwd`                             | Print the current working directory          |
| `cd /usr/local/lib`               | Change to an absolute path                   |
| `cd ../lib`                       | Change using a relative path                 |
| `cd "$HOME"`                      | Go to the home directory                     |
| `cd ..`                           | Move to the parent directory                 |
| `cd -`                            | Return to the previous working directory     |
| `pushd directory`, `popd`, `dirs` | Update, pop, and inspect the directory stack |
| `ls -a`                           | Include names beginning with a dot           |
| `ls -li`                          | Show a long listing with inode numbers       |
| `tree -d`                         | Display a directory tree when installed      |

Absolute paths identify the starting point explicitly. Relative paths are useful inside a project or another controlled working directory. Reliable commands control that starting directory and quote path values. The directory stack supports movement between several locations; `cd -` instead uses the previous-directory value, usually `$OLDPWD`.

An **inode** records a file's type, ownership, permissions, size, timestamps, and information used to locate its data. A directory entry associates a name with that inode. Inode numbers are meaningful within a filesystem. Modification time records content changes; change time records inode-status changes. **`ctime` is not creation time.** A separate birth timestamp may be available, depending on the filesystem.

A hard link creates another directory entry for the same inode. A symbolic link is a separate object containing a target path:

```bash
$ echo "content" > file1
$ ln file1 file2             # Create hard link
$ ls -li file1 file2         # Both show SAME inode number, link count = 2
$ ln -s file1 file3          # Create symbolic link
$ ls -li file1 file3         # file3 has DIFFERENT inode; shows -> file1
$ rm file1                   # Remove one hard link; file2 still works
$ cat file3                  # Dangling link! file1 is gone
```

Removing `file1` leaves the inode accessible through `file2`, while `file3` loses its target. A relative symbolic-link target is resolved from the directory containing the link. Hard links normally cannot cross filesystem boundaries or target directories; symbolic links can do both.

A zero hard-link count does not necessarily mean the data has already been reclaimed. A process with an open file descriptor can continue using an unlinked file until its remaining references are released.

For everyday file operations, `touch filename` creates an absent file or updates timestamps, `mkdir dirname` creates a directory, `mv old new` moves or renames it, and `rmdir dirname` removes an empty directory. `rm filename` removes a directory entry; it does not move the file to a desktop Trash folder. `rm -i` requests confirmation, while recursive removal needs careful checking of expanded paths.

Use `cat` for complete content, `head` and `tail` for its beginning or end, and `less` for paging. `wc` counts lines, words, or bytes according to its options. `tac` reverses line order; `rev` reverses characters within each line.

### Connect programs through standard streams

Programs conventionally inherit standard input, standard output, and standard error as file descriptors 0, 1, and 2. A file descriptor is a process-local handle to an open resource. In a terminal, input usually comes from the keyboard and both output streams appear on the display; inherited destinations can also be files or pipes.

The shell sets up redirection before executing the command:

| Syntax        | Effect                                                       |
| :------------ | :----------------------------------------------------------- |
| `cmd < file`  | Read standard input from a file                              |
| `cmd > file`  | Create or truncate a file for standard output                |
| `cmd >> file` | Append standard output                                       |
| `cmd 2> file` | Redirect standard error                                      |
| `cmd 2>&1`    | Copy standard output's current destination to standard error |
| `cmd &> file` | Redirect both output streams in Bash                         |

Order matters. In `cmd > file 2>&1`, both streams go to the file. In `cmd 2>&1 > file`, standard error retains the original output destination while standard output goes to the file. Each redirection acts on the descriptor arrangement established so far.

A pipe connects one command's standard output to the next command's standard input:

```
cmd1 | cmd2 | cmd3
```

For example:

```bash
$ cat /etc/passwd | grep root | cut -d: -f1,3 > output.txt 2>/dev/null
```

Here, `grep` selects lines containing `root`, and `cut` extracts colon-separated fields 1 and 3. This matches any line containing that text, not exclusively the account named `root`. The final redirection discards errors from `cut`; errors from earlier stages keep their own destinations.

Pipeline stages can run concurrently, avoiding explicit intermediate files. Their speed still depends on buffering, startup costs, resource contention, and backpressure from slower consumers. A pipeline therefore provides a way to combine programs without promising a fixed performance improvement.

### Search names and metadata deliberately

`locate` searches an index, making it useful for quickly finding known filename fragments. `find` walks the current filesystem and evaluates conditions such as type, name, size, and modification time. An index can miss newly created files or excluded locations; a live traversal can encounter permission errors and concurrent changes.

```bash
$ locate lfs300               # Fast DB search
$ sudo updatedb               # Refresh database as root
$ locate lfs300
```

An index refresh only covers locations permitted by its configuration. Settings such as `PRUNEPATHS` and `PRUNEFS` can exclude paths or filesystems, and update schedules depend on the distribution. `mlocate` and `plocate` are alternative implementations with their own behavior.

Useful `find` tests include `-name "*.log"`, case-insensitive `-iname`, and `-type f`, `-type d`, or `-type l` for regular files, directories, and symbolic links. `-maxdepth N` limits traversal; `-newer reference` compares modification times; `-ls` prints details. Adjacent tests normally combine as logical AND, with explicit operators available for alternatives and negation.

Quoted and unquoted patterns reach commands differently. Shell globs use `*` for a string, `?` for one character, and brackets such as `[abc]` for a character choice:

```bash
ls *.out              # All files ending in .out
ls ba?.txt
ls [p-z]*             # Files starting with p through z
du -sh *log*          # Files/dirs containing 'log' anywhere
```

The shell expands these patterns against names in the current directory before launching the program. `ba?.txt` means three characters before `.txt`, beginning with `ba`. Quoting instead delivers a literal pattern for the receiving program to interpret. Thus `find` receives `"*.log"` as a pattern, whereas `du` would treat a quoted glob as a literal pathname. Hidden-name matching, unmatched globs, and character ranges depend on shell settings and locale.

Time and size filters deserve particular care because they round differently. GNU `find` compares completed 24-hour periods for modification-time tests. For a nonnegative integer $n$:

$$
\texttt{-mtime n}: \;\; (t_{\text{now}} - t_{\text{mtime}}) / 86400 \in [n, n+1)
$$

`-mtime 7` therefore selects files from seven through just under eight days old. With $a=(t_{\text{now}}-t_{\text{mtime}})/86400$, `-mtime +n` means $\lfloor a\rfloor>n$, equivalently $a\ge n+1$. Consequently, `-mtime +7` begins at eight days. `-atime 0` means the last 24 hours rather than the current calendar day, and access-time updates can depend on mount options.

Size comparisons round upward: for a unit of $u$ bytes, `find` compares $\lceil\text{size}(p)/u\rceil$ with the requested number. For positive $n$, exact matching covers $(n-1)u<\text{size}(p)\le nu$. The suffixes `c`, `k`, `M`, and `G` mean bytes, KiB, MiB, and GiB; the default unit is 512 bytes. Accordingly, `-size -1M` matches only empty files, while `-size -1048576c` matches sizes below one MiB.

The `-exec` action substitutes a matched path for `{}`. A terminating `\;` runs the command separately for each match; `+` groups matches into batches within argument-size limits. It can still require several invocations for a large result set. `-ok` asks before each action.

```bash
$ sudo find /var/log -type f -exec grep -l "log" {} \;
$ sudo find . -size 0 -ls
$ sudo find . -newer /tmp/reffile -ls
```

Use `sudo` only when the search requires those permissions. Inspect matches before choosing a removal action, particularly for recovery files such as editor swap files. Place traversal options before tests, as in `find . -maxdepth 1 -type d`.

### Find documentation for the command in use

Finding an executable and identifying the package that installed it answer different questions. Use `type command-name` for the active shell interpretation, `dpkg -L pkg` or `rpm -ql pkg` to list installed package files, and `rpm -qf /path/to/file` to identify an installed file's owner. Package capability queries do not enumerate every program that might invoke a binary outside declared dependencies.

Local documentation often describes the installed version. Start with `help cd` for a Bash built-in, `command --help` for quick syntax, `man command` for a reference page, or `info command` for linked GNU documentation. Some minimal installations omit documentation packages, and `--help` is common rather than universal.

```bash
man topic            # Show default man page for topic
man 7 socket         # Show section 7 man page for socket
man -f topic         # List all man pages for topic (same as: whatis topic)
man -k keyword       # Search descriptions for keyword (same as: apropos keyword)
man -a topic         # Show all man pages for topic in sequence
```

Manual sections distinguish user commands (1), system calls (2), library functions (3), file formats (5), overviews (7), and administration commands (8). Explicit sections prevent ambiguity: `man 1 printf` and `man 3 printf` describe different interfaces. Default selection follows a configured search order rather than necessarily ascending section numbers.

With `less` as the pager, Space advances, `b` moves backward, `/pattern` searches, `n` finds the next match, and `q` quits. Info uses linked nodes: `n`, `p`, and `u` follow Next, Previous, and Up pointers. For commands with both built-in and external implementations, check `type`: ordinary Bash's `echo --help` prints `--help`, while `help echo` documents the built-in.

## 4. Processes, Scheduling, and Administration

### Apply administrative policy deliberately

`sudo` runs an allowed command as a target user, usually root. `su` starts a shell or command under another identity, subject to its authentication rules. Their policy and invocation models differ. Password prompts depend on configuration, and particular commands may require no prompt.

An authorized administrator should edit a sudoers fragment through `visudo -f /etc/sudoers.d/student` and validate the complete policy with `visudo -c`. Syntax checking reduces the risk of installing broken policy; ownership, permissions, and inclusion of the fragment directory still matter. The [visudo manual](https://www.sudo.ws/docs/man/visudo.man/) covers editing and validation.

A rule such as `student ALL=(ALL) ALL` grants broad permission to run commands as other users. File mode `440` means owner and group can read it; who that includes depends on ownership. A process launched through `sudo` normally receives the target user's real and effective IDs.

### Inspect process identity and state

A process is a running program instance with an address space, open resources, credentials, and execution state. Threads share many process resources while maintaining their own execution state; Linux schedules runnable tasks, including individual threads, across available CPUs.

A PID identifies a process within its PID namespace, and a PPID identifies its parent. Real and effective user and group IDs participate in permission handling, along with supplementary groups and capabilities. PIDs can be reused after exit, so verify a remembered PID before acting on it. PID 1 has a special role within its namespace; on conventional hosts it is often systemd. Orphaned children may be adopted by a designated subreaper or the namespace's init process.

| State | Meaning                                          |
| :---- | :----------------------------------------------- |
| `R`   | Running or ready to run                          |
| `S`   | Interruptible sleep while waiting for an event   |
| `D`   | Uninterruptible sleep, often associated with I/O |
| `T`   | Stopped, for example by job control              |
| `Z`   | Exited, with status awaiting collection          |

A zombie has already exited. Its parent or adopting process must collect its exit status; another termination signal cannot make it exit again. A stopped process can resume. These states describe different conditions rather than compulsory steps in a process lifecycle.

Use `ps` for a snapshot, `top` for an updating view, and `pstree` for parent-child relationships. `ps -ef` provides a broad full-format listing, `ps aux` uses BSD-style columns, and `ps -eLf` includes individual threads. `ps axo pid,user,ni,comm` selects particular columns. Lowercase `-l` requests a long format; uppercase `-L` requests thread information. Read the headings because priority displays differ between tools.

### Interpret scheduling and load

For ordinary fair-scheduled tasks, a lower nice value requests greater CPU weight relative to competitors. Values normally range from −20 to +19, with 0 the usual default. Linux uses a nonlinear weight table, with approximately multiplicative changes between adjacent nice values. CPU shares also depend on competing tasks, scheduling groups, affinity, policy, and available CPUs.

```bash
$ ps -lf                           # Show with priority (PRI) and nice (NI) columns
$ renice +5 3077                   # Increase nice (lower priority) of PID 3077
$ sudo renice -5 3077              # Root can decrease nice (increase priority)
$ nice -n 10 some-program
```

The PID is illustrative. Raising priority generally requires privilege or a configured resource-limit allowance. `nice -n 10` adds ten to the inherited nice value, yielding 10 when starting from zero. Check the local `renice` manual when scripting because argument behavior can vary.

The kernel's nice-design documentation explains weighting. Older accounts describe CFS; newer kernels have moved fair-class selection toward EEVDF, described in the kernel's EEVDF documentation. Relative weight remains useful even as the selection algorithm changes.

Linux load averages cover approximate one-, five-, and fifteen-minute timescales. These exponentially decaying averages include runnable tasks and tasks in uninterruptible sleep. They measure demand rather than CPU utilization. Dividing load by the CPU count does not reveal the percentage of CPU time used, and load above that count can reflect blocked I/O as well as runnable contention. A single CPU or constrained process group can also be saturated while system-wide load remains modest. The kernel load-average implementation documents the calculation.

Compare load with CPU percentages, process states, affinity, and workload behavior. In procps `top`, `1` toggles per-CPU statistics, `M` sorts by memory, `P` sorts by CPU use, `H` toggles threads, and `h` or `?` opens help. `k` and `r` start signal and renice prompts; `q` quits.

The header distinguishes user and system CPU time, idle time, I/O wait, interrupts, and virtual-machine steal time. Low free memory alone does not establish memory pressure because cached data can be reclaimable; consider available memory and the workload's behavior.

### Control jobs and request termination

A shell job can contain one process or a pipeline. Its job number belongs to the current shell and is distinct from a PID:

```bash
$ sleep 1000 &                     # Run sleep in background ([1] PID shown)
$ jobs                             # Shows: [1]+ Running  sleep 1000
$ jobs -l                          # Same with PID
$ fg %1                            # Bring job 1 to foreground
```

Press `Ctrl+Z` to request suspension of the foreground process group. `bg` resumes a job in the background; `fg` returns it to the foreground. `%1` addresses job 1 in the current shell's job table.

`kill %1` sends the default SIGTERM request, which a program can handle while cleaning up. `kill -9 PID` sends SIGKILL, which cannot be caught, blocked, or ignored, although a task in uninterruptible sleep may not disappear immediately. Start with ordinary termination when cleanup matters. Backgrounding alone does not guarantee survival after a login session ends.

### Schedule work with explicit environments

`sleep` delays an existing execution path, `at` queues a one-time job, and `cron` runs recurring work. GNU `sleep` accepts seconds by default and duration suffixes:

```bash
sleep 5          # Pause 5 seconds (default unit)
sleep 5m         # 5 minutes
sleep 2h         # 2 hours
```

A foreground `sleep` causes the shell to wait. By contrast, `at now + 2 hours` queues work for an installed, running `at` service. Enter commands at its prompt and submit them with `Ctrl+D`. Actual execution still depends on system availability and service behavior.

A user crontab has five time fields followed by a command:

```
MIN HOUR DOM MON DOW command
```

They specify minute (0–59), hour (0–23), day of month (1–31), month (1–12), and day of week (commonly 0–7, with 0 and 7 meaning Sunday). System crontabs and `/etc/cron.d` files commonly add a username before the command.

`30 2 * * 1 /usr/bin/backup.sh` requests execution every Monday at 02:30. `*/5 * * * * /usr/local/bin/check.sh` matches minutes 0, 5, 10, and so on. These are calendar matches rather than intervals measured from the previous job's completion. When both day-of-month and day-of-week are restricted, common implementations can trigger when either matches; consult the crontab format manual.

Use `crontab -e` to edit and `crontab -l` to inspect a user's schedule. `crontab -r` removes the entire crontab. Cron normally supplies a smaller environment than an interactive shell, so specify intended paths, working directories, and output handling. Timezone and daylight-saving behavior depend on configuration. Mail delivery of job output also requires working local mail configuration.

## 5. Network Addresses and Remote Work

### Separate addressing from transport

Network diagnosis becomes clearer when each layer has a distinct question. Ethernet and Wi-Fi carry link-layer frames; IP addresses and routes direct packets; TCP and UDP provide transport services identified by ports. Applications such as SSH and DNS clients use those services.

TCP provides an ordered, reliable byte stream or reports connection failure. UDP sends datagrams without that reliability mechanism; it does not inherently guarantee low latency. Neither protocol can overcome a permanently failed network. Ethernet CRCs, IP headers, and TCP sequence numbers serve different functions at their respective layers.

IPv4 uses 32-bit addresses, commonly displayed as four octets from 0 to 255. IPv6 uses 128 bits:

$$
N_{\text{IPv4}} = 2^{32} = 4,294,967,296 \approx 4.29 \times 10^9
$$

$$
N_{\text{IPv6}} = 2^{128} = 340,282,366,920,938,463,463,374,607,431,768,211,456 \approx 3.403 \times 10^{38}
$$

These counts describe bit patterns; both protocols reserve ranges. IPv6's larger space supports addressing without IPv4-style sharing, while routes and firewall policy still determine reachability. IPv6 traffic is not automatically encrypted.

Historical IPv4 classes A, B, and C used fixed 8-, 16-, and 24-bit network portions. Modern routing uses CIDR, which states the boundary explicitly. A `/24` prefix means 24 network bits regardless of the address's first octet.

For an IPv4 prefix length $L$:

$$
H = 32 - L
$$

$$
S_{\text{total}} = 2^H = 2^{32 - L}
$$

An ordinary subnet that reserves its network and directed-broadcast addresses has:

$$
S_{\text{usable}} = 2^{32 - L} - 2
$$

A `/24` therefore contains 256 addresses and conventionally 254 host addresses. The subtraction is not universal: `/31` point-to-point links use both addresses under RFC 3021, and `/32` represents one address.

Useful ranges include loopback `127.0.0.0/8` and private addresses `10.0.0.0/8`, `172.16.0.0/12`, and `192.168.0.0/16`. `0.0.0.0` can mean an unspecified address or wildcard bind; the prefix `0.0.0.0/0` represents the default IPv4 route.

### Inspect local configuration before remote connectivity

The `ip` utility inspects interfaces, addresses, routes, and neighbors. Older `ifconfig`, `route`, and `arp` commands may require the separate net-tools package.

| Question                         | Command                  |
| :------------------------------- | :----------------------- |
| Which addresses are assigned?    | `ip addr show`           |
| What routes are installed?       | `ip route show`          |
| Which local neighbors are known? | `ip neigh show`          |
| Which sockets are listening?     | `ss -lntup`              |
| What does the adapter report?    | `ethtool interface-name` |

`ip -br addr show` provides a compact listing, including interfaces that are not operationally up. `UNKNOWN` on loopback does not itself indicate failure. An address, link state, or listening socket answers one local question; end-to-end application connectivity requires further checks.

Persistent configuration belongs to the active network manager. NetworkManager provides `nmcli` and `nmtui`; other installations use systemd-networkd or different mechanisms. A configuration file's presence does not prove that the current service reads it.

For a failed connection, inspect the link, address, route, name resolution, and intended service separately. The default route applies when no more specific applicable route matches. Routing policy and additional tables can also affect selection.

### Distinguish DNS from application resolution

Programs using system name-service interfaces commonly follow the `hosts:` policy in `/etc/nsswitch.conf`, which may consult `/etc/hosts`, DNS, and other services. Applications can also use their own resolver, including DNS over HTTPS.

`/etc/resolv.conf` supplies resolver settings on many systems. An entry pointing to `127.0.0.53` normally identifies the local systemd-resolved stub. Use `resolvectl status` to inspect upstream servers and per-link configuration. Network services may manage the file or symlink, so direct changes may not persist.

```bash
$ host linuxfoundation.org
$ nslookup linuxfoundation.org
$ dig linuxfoundation.org +noall +answer +stats
```

These commands query DNS. `getent hosts name` is useful for checking the system resolver path, which can include sources beyond DNS. In a `dig` answer, the record's TTL is expressed in seconds, `A` identifies an IPv4 address record, and `SERVER` identifies the queried resolver. AAAA requests retrieve IPv6 address records. `+noall +answer +stats` selects output sections; `+trace` performs a different diagnostic operation.

### Read reachability and latency measurements carefully

`ping -c 3 172.16.249.129` sends three ICMP echo probes. A reply demonstrates a round trip for that traffic. Missing replies can reflect filtering or rate limiting and do not establish that every service on the target is unavailable.

`traceroute -n 8.8.8.8` varies TTL or hop limit and observes responses from intermediate devices. Some routers do not respond, and paths can change or be asymmetric. Treat the output as observations about probes rather than a guaranteed complete route for all connections.

Nmap's `-sn` performs host discovery without a port scan; older examples use `-sP`. Discovery may use ARP or several probe types, so a “ping scan” does not necessarily mean ICMP alone.

For a simple probe set without duplicate replies, packet loss is:

$$
\text{Loss Rate (\%)} = \left( \frac{P_{\text{sent}} - P_{\text{received}}}{P_{\text{sent}}} \right) \times 100
$$

When replies exist, average round-trip time is:

$$
\text{RTT}_{\text{avg}} = \frac{1}{P_{\text{received}}} \sum_{i=1}^{P_{\text{received}}} \text{RTT}_i
$$

Here, $P_{\text{sent}}$ counts probes, $P_{\text{received}}$ counts replies, and $\text{RTT}_i$ measures each round trip. For $n$ replies, iputils `ping` reports population standard deviation as `mdev`:

$$
\text{mdev} = \sqrt{\frac{1}{n} \sum_{i=1}^{n} (\text{RTT}_i - \text{RTT}_{\text{avg}})^2}
$$

For sample times of 0.412, 0.389, and 0.405 ms, the mean is 0.402 ms and `mdev` is approximately 0.0096 ms. The [iputils ping documentation](https://github.com/iputils/iputils/blob/master/doc/ping.xml) describes this statistic. Variation alone cannot identify congestion or hardware failure.

NAT translates addresses and commonly ports while tracking return-traffic mappings. Many internal hosts can share one public address, but its connection capacity cannot be derived from one simple port-count limit. Capacity depends on protocols, mapping rules, connection tuples, configured ranges, and implementation resources.

### Authenticate remote hosts and transfer data

SSH separates server identity from user authentication. A server host key identifies the destination; a user's login key authenticates the account. On a first connection, verify the host-key fingerprint through a trusted source before accepting it.

```bash
$ ssh student@172.16.249.129
$ ssh student@172.16.249.129 "df -h /home"
$ scp -r /home/student/projects student@172.16.249.129:/tmp/
$ ssh-copy-id -i ~/.ssh/id_ed25519.pub student@172.16.249.129
```

The first command opens an interactive session. The second runs `df` remotely and returns its output through SSH. The third recursively transfers a directory. OpenSSH 9.0 switched `scp` to SFTP by default; `-O` requests the legacy protocol when needed.

Use an unused path when generating an additional identity to preserve existing keys. For an interactive personal key, a passphrase and SSH agent can protect the stored key while avoiding repeated entry during an active session. `ssh-copy-id` installs the public key after authentication; it neither copies the private key nor disables password login. Allowed login methods remain a separate server policy.

For web retrieval, `wget` normally saves content to disk and `curl` normally writes it to standard output:

```bash
$ wget https://example.com/archive.tar.gz
$ wget -r -np -k https://example.com/docs/
$ curl https://freecodecamp.org
$ curl -o saved_page.html https://freecodecamp.org
$ curl -I https://linuxfoundation.org
```

The `example.com` URLs are placeholders. For `wget`, `-r` follows links recursively, `-np` prevents ascent to parent paths, and `-k` adjusts downloaded links for local use. `-c` can resume supported partial transfers. For `curl`, `-o` saves to a named file, `-L` follows redirects, and `-I` requests HTTP headers using HEAD, which a server may handle differently from GET.

These tools retrieve data without rendering pages or executing their JavaScript. They fit naturally into command-line workflows for inspecting headers, downloading archives, and passing responses to other programs.

## 6. Storage, Synchronization, and Editors

Working with files connects several separate decisions: where data lives, how changes are reviewed, how copies are preserved, and how text is edited. A path may lead to local storage or a network filesystem, but the same familiar commands often apply. Understanding the boundaries between mounting, copying, synchronization, archiving, and editing prevents a convenient operation from being mistaken for a complete preservation workflow.

### Identify contents before trusting a filename

A filename extension is a convention, not proof of format. The `file` utility examines content and other information to identify likely types. It can distinguish a shell script, PDF, compressed archive, or image without relying only on the suffix:

```bash
$ file script.sh document.pdf archive.tar.gz image.png
```

Renaming an executable does not turn it into plain text. A disposable copy demonstrates this without moving the installed system command:

```bash
example_dir=$(mktemp -d)
cp /bin/ls "$example_dir/test.txt"
file "$example_dir/test.txt"
```

The copy remains an ELF executable despite its `.txt` name. Execution also requires a supported format or interpreter, appropriate path access, and permission under mount and security rules. An executable bit alone does not satisfy all those requirements. Conversely, an explicitly invoked interpreter can read a shell script that lacks permission for direct execution. Desktop applications may still use extensions to select an application, so opening behavior and the file's actual contents are separate questions.

### Mount storage and interpret capacity

A mount attaches a filesystem to the directory tree. Mounting at `/mnt/data` exposes that filesystem's root at the chosen path. Existing files beneath the mount point become hidden until it is unmounted; they remain on their original filesystem and continue consuming its space. Placing `/home` or `/var` on separate filesystems can contain capacity problems, although applications still fail when the particular filesystem they need becomes full.

`df -Th` reports mounted filesystem types and readable capacity figures. The following mount assumes that `/dev/sdb1` has already been identified and contains an ext4 filesystem. Mounting does not format it.

```bash
$ df -Th
$ sudo mkdir -p /mnt/data
$ sudo mount -t ext4 /dev/sdb1 /mnt/data
$ sudo umount /mnt/data
```

The unmount command is spelled `umount`. It can fail while processes use the filesystem, including a shell whose working directory lies inside it. Resolve those uses before removing storage. Network filesystems join the same directory tree: NFS commonly exports Unix directories, while SMB shares can be mounted through Linux's CIFS support. Access then depends on server permissions and network availability as well as the local path.

Persistent definitions belong in `/etc/fstab`. Its six fields identify the source, mount point, filesystem type, options, historical `dump` setting, and filesystem-check order. UUIDs identify filesystems independently of device enumeration; the identifiers here are examples:

```
# <device/UUID>                         <mount_point>  <type>  <options>       <dump>  <pass>
UUID=3b8f2d1e-8e54-4a2e-9d22-1234567890ab  /              ext4    defaults        1       1
UUID=4c9a1d2e-3f65-4b1a-8c11-0987654321fe  /home          ext4    defaults,noatime 0       2
/dev/sda3                               none           swap    sw              0       0
```

Options including `ro`, `rw`, `noexec`, `nosuid`, `nodev`, and `noatime` change mount behavior. Zero disables the fifth field's dumping request. The sixth field traditionally uses `1` for root, `2` for other checked filesystems, and `0` to skip checks, with behavior also depending on the filesystem and boot tools.

Capacity figures need interpretation. Some filesystems withhold free blocks from ordinary users, so reported used and available space may not sum to the total. A simplified accounting model is:

$$
U_{\text{total}} = U_{\text{used}} + U_{\text{avail}} + U_{\text{reserved}}
$$

Here, the reserved term means free space excluded from the reported available figure. It is not necessarily the full configured reservation. Utilization can therefore be calculated as:

$$
\text{Usage Percentage (\%)} = \left( \frac{U_{\text{used}}}{U_{\text{used}} + U_{\text{avail}}} \right) \times 100
$$

Human-readable rounding can obscure this relationship. Ext4 supports configurable reserved blocks, but there is no universal Linux rule that every filesystem reserves five percent. A reserve can leave administrative room during a storage incident without guaranteeing that every service continues working.

### Review differences before replacing files

`diff` compares text line by line. Its unified format, selected with `-u`, expresses changes as hunks containing context, removed lines, and added lines. Each hunk header identifies the old and new starting positions and line counts; context helps locate a change after nearby lines move. `diff -r` compares directory trees. Options that ignore case or whitespace are appropriate only when those differences are irrelevant.

A header has the form `@@ -s1,l1 +s2,l2 @@`: the first pair describes the old file, and the second describes the new file. The body must contain the declared line counts, counting context on both sides. A shortened illustration with an unchanged header is therefore unsuitable for direct application as a patch.

`cmp` compares bytes and normally reports the first difference. `diff3` compares three versions, useful when two edited files share a common base. `patch` applies a suitable diff. A simple review and application sequence is:

```bash
$ diff -u main_v1.c main_v2.c > fix_null_pointer.patch
$ cat fix_null_pointer.patch
$ patch --dry-run main_v1.c fix_null_pointer.patch
$ patch main_v1.c fix_null_pointer.patch
```

Generate the patch from actual files and inspect it before applying it. `patch -p1` removes one leading pathname component and is useful for paths such as `a/src/main.c`; it is unsuitable for the bare filenames above. Successful application establishes that text was changed. Relevant program checks still need to establish that the changed program behaves correctly. Rejected or unexpectedly located hunks deserve investigation.

### Synchronize against the destination you previewed

`cp -a` copies a tree while preserving the attributes supported by the implementation and available privileges. `rsync` adds selective updates, direct remote destinations, and previews. It normally uses size and modification time to decide which files need updates. Remote transfers can reuse matching destination data through its delta algorithm; local transfers commonly copy whole files.

Archive mode does not include every attribute: hard links, ACLs, and extended attributes require options such as `-H`, `-A`, and `-X`. Interrupted-transfer handling also depends on options, including `--partial`.

Preview the operation using the same source, destination, and deletion options as execution:

```bash
$ rsync -avzn --delete /home/student/projects/ backupuser@192.168.1.50:/var/backups/projects/
$ rsync -avz --delete /home/student/projects/ backupuser@192.168.1.50:/var/backups/projects/
```

The `n` requests a dry run. The trailing slash on `projects/` copies its contents into the destination. `--delete` removes destination entries that no longer exist in the source, so a preview aimed at another destination cannot validate this operation.

A mirror can propagate accidental deletion or corruption. Recoverable backups also need retained versions or snapshots, suitable consistency for live application data, and restoration checks. Transfer efficiency answers how data moves; retention determines which earlier state remains available after a mistake.

### Archive and compare compression

An archive collects files; compression reduces a byte stream when its contents permit it. `tar` can package a directory tree, after which `gzip`, `bzip2`, or `xz` can compress the same archive. Gzip uses DEFLATE, bzip2 uses Burrows–Wheeler-based compression, and xz commonly uses LZMA2. Their practical tradeoffs involve output size, CPU time, memory, settings, and input data.

Keeping the original with `-k` lets each compressor receive the same input:

```bash
$ tar -cf data.tar /var/log/
$ gzip -k data.tar
$ bzip2 -k data.tar
$ xz -k data.tar
$ ls -lh data.tar*
```

Reading `/var/log` may require additional permission, and archiving actively changing logs does not necessarily create a consistent snapshot. There is no fixed compression ratio or speed ranking for every file. Small or already compressed inputs can grow.

For nonempty input, the ratio and corresponding byte savings are:

$$
R_{\text{comp}} = \frac{S_{\text{uncompressed}}}{S_{\text{compressed}}}
$$

$$
\text{Space Savings (\%)} = \left( 1 - \frac{S_{\text{compressed}}}{S_{\text{uncompressed}}} \right) \times 100 = \left( 1 - \frac{1}{R_{\text{comp}}} \right) \times 100
$$

A ratio of four means 75% savings. A ratio below one means the output grew. These formulas compare file lengths; filesystem allocation and the extra space used by retaining several compressed variants are separate considerations.

For example, compressing a 100 MB archive to 28 MB saves 72% of its original byte count; outputs of 21 MB and 15 MB save 79% and 85%. Such results describe the particular archive and settings tested. Compare the tools using the data that will actually be stored, especially when processing time or available memory constrains the choice.

### Create plain text and choose an editor

Scripts and configuration files usually require plain text. Editors preserve that format, though encoding and line endings still matter. UTF-8 can use multiple bytes per character, so character and byte counts differ. A word processor must explicitly export plain text to produce a file that a configuration parser can read.

Shell redirection is sufficient for short files: `>` creates or truncates, while `>>` appends. With `cat > myfile.txt`, pressing `Ctrl+D` on an empty input line indicates end of input. A quoted here-document delimiter keeps its body literal:

```bash
cat << 'EOF' > setup.sh
#!/bin/bash
echo "Starting system configuration..."
EOF
```

The closing delimiter starts its input line. Quoting the opening delimiter prevents variable and command expansion inside the body. Redirection still overwrites an existing file and does not make the result executable.

Nano provides visible shortcut hints, typically `Ctrl+O` to save, `Ctrl+X` to exit, and `Ctrl+G` for help. Graphical editors such as Gedit and KWrite offer familiar menus and tabs. Vim organizes editing around modes and composable commands; Emacs provides commands operating on buffers within a programmable environment. Programs may consult `EDITOR` or `VISUAL`, with precedence determined by each application. Installed availability matters: even a Vi-compatible editor may be absent from a minimal container.

### Use Vim's command structure

Vim separates text entry from commands. `i` enters Insert mode; `Esc` returns to Normal mode. From Normal mode, `:` starts an Ex command, `/` searches forward, and `?` searches backward. Typing a colon while still in Insert mode simply inserts that character.

Operators combine with motions: `d` deletes, `c` changes, and `y` yanks. Thus `d$` deletes to the end of the line, `3dw` deletes across three word motions, and `c2w` changes two words before entering Insert mode. Counts before an operator and a motion multiply. Text objects provide other targets, such as `i)` for the contents of parentheses.

The same motions support navigation: `h`, `j`, `k`, and `l` move by direction; `w`, `b`, and `e` move among words; `0`, `^`, and `$` select line positions. `gg` moves to the beginning and `G` to the end of a file. Learning these targets makes both navigation and composed editing commands easier to remember.

Useful direct commands include `dd` for a line deletion, `yy` for a line yank, `p` or `P` to put text, `u` to undo, and `Ctrl+R` to redo. `:w` saves, `:q` quits when permitted, `:wq` saves and quits, and `:q!` abandons changes to the buffer being left. A whole-buffer replacement with confirmation is:

```vim
:%s/listen 80;/listen 8080;/gc
```

Here `%` selects the buffer, `g` replaces all matches on each line, and `c` asks for confirmation. External commands require another distinction: `:!wc %` counts the saved file, including none of the buffer's unsaved changes. `:%!sort` filters the entire current buffer through an external sort. Filenames expanded into shell commands require appropriate escaping.

Swap files can help recover interrupted edits. A warning can also mean another editing session remains active, so inspect the situation before removing recovery data.

### Understand Emacs buffers and saving

An Emacs buffer holds working content, a window displays a buffer, and a frame contains windows. Splitting the display does not duplicate the file. `C-` means Control; `M-` means Meta, often Alt or an Escape prefix.

`C-x C-f` opens a file, `C-x C-s` saves the current buffer, and `C-h t` opens the tutorial. `C-x 2` creates windows above and below each other, `C-x 3` places them side by side, and `C-x o` switches between them. `C-x b` changes buffers.

`C-Space` sets the mark; moving point defines a region. `M-w` copies it, `C-w` kills it, and `C-y` yanks saved text back. `C-s` and `C-r` search incrementally; `M-%` starts query replacement. For several modified files, `C-x s` offers to save their buffers, while `C-x C-c` handles exit and prompts about unsaved work.

Prefix arguments change the scale or behavior of many commands. For example, `C-u 10 C-n` requests movement by ten lines. Repeated `C-u` supplies values of four, sixteen, and so on, but some commands interpret an argument as a behavior change rather than repetition.

## 7. Accounts, Environment, and Permissions

Files carry numeric user and group ownership, while processes carry identities and supplementary groups used for access checks. Account names make these numbers manageable. UID 0 identifies root; separate service identities help isolate applications. Ranges such as 1–999 for system accounts and 1000 upward for regular users are distribution conventions, with exceptions determined by local policy.

### Read account records and manage membership

Local account information commonly lives in three files, although directory services can supply additional identities. `/etc/passwd` contains seven colon-separated fields:

```
username:x:UID:GID:Comment/GECOS:HomeDirectory:LoginShell
student:x:1000:1000:Student Account:/home/student:/bin/bash
```

The `x` conventionally points to `/etc/shadow`, which holds restricted password hashes, aging information, and password-lock state. Permissions and hash formats depend on the system. `/etc/group` stores group names, GIDs, and supplementary members:

```
groupname:x:GID:user1,user2,user3
developers:x:1005:student,edolphy
```

A user whose primary GID matches a group need not appear in its member list. `/etc/skel` supplies home-directory templates, commonly including shell dotfiles. Account defaults also depend on `/etc/default/useradd`, `/etc/login.defs`, and distribution tooling.

Access to shadow records is determined by actual ownership and permissions. A mode such as `0640` includes read permission for the owning group as well as the owner. Treat local account files as one source of identity information, since configured directory services can resolve accounts absent from those files.

An account and project-group setup can use:

```bash
$ sudo useradd -m -s /bin/bash -c "Eric Dolphy" edolphy
$ sudo passwd edolphy
$ sudo groupadd audio-engineers
$ sudo usermod -aG audio-engineers edolphy
$ id edolphy
```

`-m` requests a home directory, `-s` selects the shell, and `-c` supplies a description. `usermod -aG` appends supplementary membership; omitting `-a` replaces that list. Existing processes normally keep their current credentials, so a new login session is the usual way to acquire changed group membership.

Account retirement involves more than removing a record. `userdel -r` normally removes the home directory and mail spool, leaving files owned by that UID elsewhere, remote data, and backups. Password locking with `passwd -l` or `usermod -L` may leave authentication through SSH keys available. Inspect the identity's data and access methods when retiring it. Group deletion is also constrained when the group remains someone's primary group.

For administration, `su` starts a command or shell under another user, and `su -` requests a login-style environment. `sudo` runs a permitted command under a target identity according to policy. Authentication and logging are configurable for both. A scoped administrative command makes the effect easier to review than an unrestricted root session, in which every subsequent command carries broader authority.

### Place shell settings where they will run

Bash startup depends on invocation:

| Invocation                    | Startup behavior                                                                                   |
| :---------------------------- | :------------------------------------------------------------------------------------------------- |
| Login shell                   | Reads `/etc/profile`, then the first readable `~/.bash_profile`, `~/.bash_login`, or `~/.profile`. |
| Interactive, non-login shell  | Reads `~/.bashrc`.                                                                                 |
| Ordinary noninteractive shell | Can read the file named by `BASH_ENV`.                                                             |

A profile can explicitly source `.bashrc`. Distribution files such as `/etc/bashrc`, `/etc/bash.bashrc`, and `/etc/profile.d/*.sh` fit into local configuration rather than one universal sequence.

Put login environment settings in the profile that actually runs and interactive aliases or functions in the interactive configuration. An existing `.bash_profile` can prevent Bash from selecting `.profile`, explaining why a setting placed there never takes effect.

A shell variable belongs to the current shell. Exporting it includes it in the environment of subsequently launched programs. Assuming the variable was not already exported:

```bash
$ MY_PROJECT="Apollo"
$ bash -c 'echo "Child sees: $MY_PROJECT"'
Child sees:
$ export MY_PROJECT="Apollo"
$ bash -c 'echo "Child sees: $MY_PROJECT"'
Child sees: Apollo
```

A parenthesized subshell behaves differently: it inherits shell state, including nonexported variables. A child's later assignments do not update its parent. `VAR=value command` provides an environment assignment for that external command's invocation.

`env` and `printenv` inspect environment variables; Bash's `set` also displays shell variables and functions. `HOME` identifies the home directory, `PATH` supplies command-search directories, and `PS1` sets the primary prompt. `SHELL` describes the configured login shell and does not reliably identify the shell executing the current command.

`PATH` is searched in order. Including the current directory lets a local filename compete with installed commands; `./program` names that local program explicitly. History is separate state: `history` displays commands, `Ctrl+R` searches them, and `HISTSIZE`, `HISTFILESIZE`, and `HISTFILE` control retention and storage. Configuration determines when and how the history file is written.

### Distinguish file access from directory access

Traditional permissions assign owner, group, and other access bits. The first character in `ls -l` identifies file type; the next nine encode three `rwx` groups. For example, `rwxr-xr--` grants all three bits to the owner, read and execute to the group, and read to others.

| Bit | Regular file                                            | Directory                                        |
| :-- | :------------------------------------------------------ | :----------------------------------------------- |
| `r` | Read contents.                                          | List entry names.                                |
| `w` | Change contents.                                        | Modify entries, normally with search permission. |
| `x` | Permit direct execution, subject to other requirements. | Search names and traverse paths.                 |

Deleting a file generally depends on its containing directory's permissions, rather than the file's own write bit. Sticky bits and other controls can impose further restrictions. Similarly, seeing a filename in a listing does not establish permission to resolve the path and access its contents.

`chmod` accepts symbolic changes, such as `chmod u+x,g-w,o=r file.txt`, or octal modes. Read, write, and execute have weights four, two, and one. For each permission tier:

$$
D_t = 4 \cdot \mathbb{I}(r_t) + 2 \cdot \mathbb{I}(w_t) + 1 \cdot \mathbb{I}(x_t)
$$

The indicator is one when a bit is set and zero otherwise. Thus `755` represents `rwxr-xr-x`, `644` represents `rw-r--r--`, `600` restricts reading and writing to the owner, and `700` additionally permits owner execution or directory traversal. These are octal digit sequences. Setgid and sticky bits are additional to the nine ordinary bits.

`chown` changes ownership and `chgrp` changes group ownership. `sudo chown student:developers file.txt` changes both together. Recursive operations apply throughout a tree and should be scoped to the intended directory.

### Understand permission selection and umask

Ordinary mode checks choose one permission class. A matching owner uses owner bits; otherwise, matching primary or supplementary group membership selects group bits; otherwise, other bits apply. The classes are not added together. An owner denied access cannot fall through to more permissive group bits.

Actual Linux authorization also involves filesystem IDs, capabilities, ACLs, security policy, and mount restrictions. Filesystem IDs normally track effective IDs. A read-only mount can prevent a write despite otherwise permissive modes, and overriding ordinary execution checks on a regular file still requires at least one execute bit.

For newly created objects without a default ACL, umask clears bits from the mode requested by the application. `&` means bitwise AND and `~` complements the mask. Applications commonly request `0666` for files and `0777` for directories. With umask `0022`, the ordinary bits become:

$$
\begin{aligned}
M_{\text{file}} &= 0666 \ \& \ (\sim 0022) = 0666 \ \& \ 0755 = 0644 \ (\texttt{rw-r--r--}) \\
M_{\text{dir}} &= 0777 \ \& \ (\sim 0022) = 0777 \ \& \ 0755 = 0755 \ (\texttt{rwxr-xr-x})
\end{aligned}
$$

The complement shown is limited to the nine ordinary permission bits. Umask `0077` similarly yields `0600` and `0700` from those base modes. Umask is a bit-clearing operation, not arithmetic subtraction, and does not change existing files. Applications may request other modes; a parent's default ACL changes creation rules.

For an existing project tree, separate directory and regular-file modes avoid giving every data file execute permission:

```bash
$ sudo mkdir -p /srv/audio_project
$ sudo chown -R edolphy:audio-engineers /srv/audio_project
$ sudo find /srv/audio_project -type d -exec chmod 775 {} +
$ sudo find /srv/audio_project -type f -exec chmod 664 {} +
$ chmod 600 ~/.ssh/id_ed25519
```

The account and group must still exist. These modes allow group modification and also allow other users to read files and traverse directories. The file operation removes existing execute bits, so use it only where intended. Current ownership and modes do not establish a future-file policy: setgid directories can supply inherited group ownership, while umask or default ACLs shape new permissions.

## 8. Text Processing and Useful Pipelines

An editor supports deliberate changes to particular passages. Repeated transformations are often easier to express as commands connected by pipes. A pipe sends one program's standard output into the next program's standard input; standard error remains separate unless redirected. Developing a pipeline one stage at a time makes its assumptions visible and lets intermediate results be checked.

### Inspect input and choose matching rules

`cat` concatenates files; `tac` reverses record order. `less` browses text interactively, with `-N` for line numbers, `-S` for long display lines, `/pattern` for searching, and `+F` for following appended data. `head -n 20` selects the first twenty lines, while `tail -n 20` selects the last twenty and `tail -f` follows growth.

`wc -l`, `wc -w`, and `wc -c` count newlines, words, and bytes respectively. Character counting is separate from byte counting. A final record without a terminating newline does not add one to `wc -l`, even when a reader sees it as the file's last line.

Many filters process records incrementally, but a pipeline does not guarantee constant memory. A simple line transformation may use memory proportional to the longest record. Awk arrays can grow with distinct keys, sed can accumulate multiline data, and sort needs memory and often temporary files to order input. Choose tools according to their actual operation and resource use.

Regular expressions describe text patterns. `grep` and `sed` use basic syntax by default; `grep -E`, `sed -E`, and awk provide extended syntax. In extended expressions, `+`, `?`, `|`, and parentheses act as operators. GNU tools support some escaped extended operators in basic mode, but that behavior is not universal.

Quote expressions so the shell does not interpret them first. In ordinary line-based use, `^` and `$` anchor the beginning and end of a line, while `[[:space:]]` denotes a character class. Regular expressions differ from filename globs; identify which program receives and interprets each pattern.

### Transform lines with sed

Sed normally reads a line into pattern space, executes its commands, and prints the result. Substitution follows `s/pattern/replacement/flags`: `g` replaces every match on the line, while `p` prints after a successful substitution and is commonly paired with `-n` to suppress automatic printing. Addresses restrict a command to selected lines or patterns.

```bash
$ sed -e 's/is/are/' input.txt
$ sed '1,2s:is:are:g' input.txt
$ sed -e '/^[[:space:]]*#/d' -e '/^[[:space:]]*$/d' /etc/nginx/nginx.conf
```

The first example replaces only the first matching sequence per line, including matches inside longer words. The second changes every match on lines one and two, using a colon delimiter. The third removes blank lines and lines whose first nonspace character is `#`; it selects text rather than parsing the configuration language.

Command order matters. A `d` command immediately ends the current cycle, while hold space, branches, and multiline commands introduce additional state.

GNU sed's `-i` writes a temporary file and replaces the original pathname. That can break hard-link relationships and, by default, replace a symlink instead of editing its target. `--follow-symlinks` changes the latter behavior. Before using an in-place edit, understand which path will change and whether the operation preserves the relationships that matter.

### Extract fields and summarize with awk

Awk divides records into fields and evaluates pattern-action rules. Records default to lines and fields use whitespace separation; `-F` selects another separator. `$0` is the complete record, `$1` the first field, `NF` the field count, and `$NF` the final field. `NR` counts processed records across input files, while `OFS` separates comma-separated output expressions.

`BEGIN` initializes before input, ordinary rules run on matching records, and `END` summarizes afterward:

```awk
  BEGIN { print "=== REPORT HEADER ===" }    # Executes ONCE before reading input
  /pattern/ { print $1, $3 }                  # Executes on matching lines
  { total += $2 }                             # Executes on EVERY line
  END { print "Total Sum:", total }          # Executes ONCE after processing all lines
```

Rules execute in order and can change state or the current record, affecting later rules. Statements such as `next` and `exit` alter the ordinary flow. A practical account-field extraction is:

```bash
$ awk -F: '{ print "User: " $1 "\t Shell: " $7 }' /etc/passwd
```

Filtering accounts by UID requires local policy; a threshold alone does not reliably classify every human and service account. Likewise, summing an `ls -l` size column does not measure all storage used by an owner. Hidden files, nested content, filenames, and output formats complicate that approach. Use `du` for allocated filesystem usage and select files explicitly for owner-specific reports.

Numeric input also has a locale contract. Awk implementations and modes differ in their use of decimal separators. `LC_ALL=C awk '...'` can enforce C-locale conventions where appropriate, but it does not convert comma-decimal data into that format.

### Sort, count, and combine records

`sort` orders records using selected keys and locale. `-n` requests numeric comparison, `-r` reverses order, `-k 3,3n` selects a numeric third-field key, and `-t` changes the delimiter. `sort -u` removes duplicate sort keys. The C locale requests bytewise ordering where that is the intended comparison.

`uniq` combines adjacent equal lines. Its `-c` counts runs, `-d` selects repeated runs, and `-u` selects runs occurring once. Sorting first groups identical values for global counts; skip sorting when the original adjacent runs are what matters.

Assuming a web log's first field is the client address, this pipeline finds the five most frequent values:

```bash
$ awk '{ print $1 }' /var/log/nginx/access.log | sort | uniq -c | sort -nr | head -5
```

Awk extracts addresses, the first sort groups them, `uniq -c` counts them, the second sort ranks counts, and head selects five results. The numeric comparison applies to counts, not to the components of IP addresses. Verify the log format before treating the result as a traffic report.

`paste -d, names.txt phones.txt > roster.csv` pairs corresponding lines with a comma. Inputs must already have matching order; paste does not join identities. `cut -d: -f1,7 /etc/passwd` extracts simple delimiter-separated fields. For a match by key, `join` combines records sharing a field after both inputs are sorted with compatible ordering.

Neither comma-delimited paste nor cut implements full CSV quoting. Embedded commas, quotes, or newlines require a CSV-aware parser. Field selection is dependable only when the input format matches the command's assumptions.

### Search text and inspect binary strings

Grep selects matching lines. `-v` inverts selection, `-n` adds line numbers, `-c` counts matching lines, and `-l` lists matching filenames. GNU grep's `-r` and `-R` both traverse directories but differ in symlink handling. Uppercase `-I` skips binary matches; lowercase `-i` makes matching case-insensitive.

```bash
$ grep -rniI "database connection failed" /var/log/app/
$ grep -Ev '^[[:space:]]*(#|$)' /etc/ssh/sshd_config
$ strings /bin/ls | grep -E "(GPL|GNU|Free Software)"
```

The configuration filter removes blank or whitespace-only lines and comments preceded by optional whitespace. It does not validate SSH configuration syntax. `strings` extracts runs of printable characters from binary data, commonly requiring at least four characters. Encoding and scanning options affect results. A matched string offers a clue about contents without establishing that a particular code path runs or that the program is safe.

### Normalize, capture, and divide streams

`tr` translates or deletes characters from standard input. `tr 'a-z' 'A-Z'` performs a simple range-based conversion, and `tr -s ' '` squeezes repeated spaces. Locale and multilingual text can affect character-range behavior. `tr -d '\r'` removes every carriage return, including meaningful embedded characters, so a line-ending conversion needs to match the intended input contract.

`tee` writes input both to a file and onward to standard output. For example, `command | tee log.txt | grep "ERROR"` preserves the full standard-output stream while displaying selected lines. Tee overwrites by default and appends with `-a`. Standard error enters that saved stream only when redirected into the pipeline.

`split -l 1000 input.txt chunk_` divides text by line count. `split -b 50M large.iso part_` divides by bytes; GNU split treats `M` as a power-of-two suffix, so full chunks contain 50 MiB. For positive line count and chunk limit. All chunks except possibly the last reach the line limit. Equal line counts do not imply equal byte sizes, and empty input needs no final chunk.
