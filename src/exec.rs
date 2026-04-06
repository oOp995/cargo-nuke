use colored::{Colorize};

use crate::cargos::{self, Cargos, SearchCriteria};
use crate::{cli::ArgsList, utils};
use std::thread;
use std::{error::Error, io, time::Duration};

///top-priorirty function
/// if arguments and options conflict return error
pub fn is_conflict(arg: &ArgsList) -> Result<(), Box<dyn Error>> {
    if arg.conflict.is_conflict() {
        if let Some(e) = arg.conflict.reason() {
            let e_s=e.to_string();
            eprintln!("{}",e_s.red());
            return Err(Box::new(e));
        };
    }
    Ok(())
}

pub fn execute(arg: &ArgsList) -> Result<(), Box<dyn Error>> {
    //second priority is dry-run
    if arg.is_dry() {
        println!("{}","forced dry-run".yellow());
        execute_dry(arg)?;

        return Ok(());
    } else {
        //sure is third priority
        if arg.is_sure() {
            println!("{}","sure-flag is switched on".red());
            execute_sure(arg)?;
        } else {
            // not-sure flagged is the 4th priority
            execute_normally(arg)?;
        }
    }

    Ok(())
}
fn execute_dry(args: &ArgsList) -> Result<(), Box<dyn Error>> {
    execute_common(args)?;

    println!("{}","task aborted due to dry run".yellow());
    Ok(())
}

fn execute_sure(arg: &ArgsList) -> Result<(), Box<dyn Error>> {
    let cargos = execute_common(arg)?;
    let matched_cargos = cargos.cargo_count();
    if matched_cargos > 0 {
        let size_reclaimed = utils::clean_batch(&cargos.target_path)?;
        println!("{}","cargo nuke completed".green());

        let reclaimed=format!("{:.2}",size_reclaimed);
        println!("{}{}{}","Freed ".green(),reclaimed.green()," MB of disk space".green());
        
    }
    Ok(())
}

fn execute_normally(arg: &ArgsList) -> Result<(), Box<dyn Error>> {
    let cargo = execute_common(arg)?;
    let count = cargo.cargo_count();
    if count > 0 {
        println!("{}{}{}",
            "Are you sure you want clean all ".yellow(),count.to_string().yellow()," crate artifacts ?".yellow());
        
        println!("{} / {}",
        
            "Y To proceed".green(),
            "else to cancel".red(),
        );
        request_confirmation()?;
        let size_recliamed = utils::clean_batch(&cargo.target_path)?;
        let reclaimed=format!("{:.2}",size_recliamed);
        println!("{}{}{}","Freed ".green(),reclaimed.green()," MB of disk space".green());
    }

    Ok(())
}

fn execute_common(arg: &ArgsList) -> Result<Cargos, Box<dyn Error>> {
    
    let path = arg.path();
    {
        let crit_period = Duration::from_hours(arg.days() * 24);
        let criteria = SearchCriteria::new(crit_period, 0);
        let searching=format!("Searching -> {:?}",path);
        println!("{}",searching.blue());
        let cargos_se = cargos::Cargos::from_dir(path, &criteria)?;
        let modi = &cargos_se.last_build;
        let mut total_reclaim = 0;
        let _ = &cargos_se
            .crates
            .iter()
            .for_each(|(_, s)| total_reclaim += s);
        if cargos_se.cargo_count() == 0{
            println!("{}","no cargos found".green());
            return Err("CargosNotFound".into());
        }
        println!("{}","Crates found matches search criteria:".green());
        cargos_se.crates.iter().for_each(|(n, s)| {

            println!(
                "{} | build_size: {} | last_build: since {} days",
                n.green(),
                s.to_string().green(),
                (modi.get(n).unwrap() / 24).to_string().green()
            );
            thread::sleep(Duration::from_millis(50));
        });

        println!("\ntotal-of {} cargo's found", cargos_se.cargo_count().to_string().yellow());
        println!("total-size reclaimable: || {} || MB ", total_reclaim.to_string().green());

        Ok(cargos_se)
    }
}

pub fn request_confirmation() -> Result<bool, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().parse::<String>() {
        Ok(s) if s.eq("Y") => {}
        _ => {
            println!("{}","user aborted process".green());
            return Err("Proccess aborted by user".into())
        },
    }

    Ok(true)
}
