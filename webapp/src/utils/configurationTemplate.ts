export const configurationTemplate = "# the delivery service configuration file" +
    "\n" +
    "# message time to live in seconds. The delivery service promises to keep the message for at least this long. \n" +
    "# 15811200 is 6 months\n" +
    "messageTTL: 15811200\n" +
    "\n" +
    "# the maximum size of a message in bytes, 10000000 is roughly 10MB \n" +
    "sizeLimit: 10000000\n" +
    "\n" +
    "# uncomment the next block and replace all placeholders with your information in order to enable email notifications\n" +
    "#notificationChannel:\n" +
    "# - type: EMAIL\n" +
    "#   config:\n" +
    "#    smtpHost: <place your email provider's url here, e.g. smtp.gmail.com>\n" +
    "#    smtpPort: <place your port here, default is 587>\n" +
    "#    smtpEmail: <place the email address here>\n" +
    "#    smtpUsername: <place your user name here>\n" +
    "#    smtpPassword: <place your password here>\n";