use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    command::CommandOwner,
    commands::{login::LoginCommandOwner, register::RegisterCommandOwner, totp::TOTPCommandOwner},
};

#[derive(Clone)]
pub struct Context {
    commands: Rc<RefCell<HashMap<String, Box<dyn CommandOwner>>>>,
}

impl Context {
    pub fn new() -> Self {
        let commands: Rc<RefCell<HashMap<_, Box<dyn CommandOwner>>>> =
            Rc::new(RefCell::new(HashMap::new()));

        let login = LoginCommandOwner::new();
        commands
            .borrow_mut()
            .insert(login.get_command_name().to_string(), Box::new(login));

        let register = RegisterCommandOwner::new();
        commands
            .borrow_mut()
            .insert(register.get_command_name().to_string(), Box::new(register));

        let totp = TOTPCommandOwner::new();
        commands
            .borrow_mut()
            .insert(totp.get_command_name().to_string(), Box::new(totp));

        Self { commands }
    }

    pub fn commands(&self) -> &RefCell<HashMap<String, Box<dyn CommandOwner>>> {
        self.commands.as_ref()
    }

    pub fn commands_mut(&mut self) -> &mut Rc<RefCell<HashMap<String, Box<dyn CommandOwner>>>> {
        &mut self.commands
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
